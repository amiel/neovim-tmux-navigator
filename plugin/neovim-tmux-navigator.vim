" Initialize the channel
if !exists('s:jobId')
	let s:jobId = 0
endif

" Constants for RPC messages.
let s:Up = 'up'
let s:Down = 'down'
let s:Left = 'left'
let s:Right = 'right'

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:command = ['neovim-tmux-navigator', 'server']

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "neovim-tmux-navigator: cannot start rpc process"
  elseif -1 == id
    echoerr "neovim-tmux-navigator: rpc process is not executable"
  else
    " Mutate our s:jobId variable to hold the channel ID
    let s:jobId = id 
    
    call s:configureCommands()
  endif
endfunction

" NOTE: These are not used yet"
function! s:configureCommands()
  command! NvimTmuxNavigatorUp :call rpcnotify(s:jobId, s:Up)
  command! NvimTmuxNavigatorDown :call rpcnotify(s:jobId, s:Down)
  command! NvimTmuxNavigatorLeft :call rpcnotify(s:jobId, s:Left)
  command! NvimTmuxNavigatorRight :call rpcnotify(s:jobId, s:Right)
endfunction

function! s:OnExit(job_id, data, event) dict
  lua require("notify")("neovim-tmux-navigator on exit")
  echo "neovim-tmux-navigator: rpc process stopped: id=" .. a:job_id .. " event=" .. a:event .. " -- " .. string(a:data)
endfunction

function! s:OnError(job_id, data, event) dict
  lua require("notify")("neovim-tmux-navigator on error")
  echo "neovim-tmux-navigator: rpc process on error: id=" .. a:job_id .. " event=" .. a:event .. " -- " .. string(a:data)
endfunction


" Initialize RPC
function! s:initRpc()
  if s:jobId == 0
    let jobid = jobstart(s:command, { 'rpc': v:true, 'on_exit': function('s:OnExit'), 'on_stderr': function('s:OnError') })
    echo "neovim-tmux-navigator: started rpc process"
    let g:jobId = jobid
    return jobid
  else
    lua require("notify")("neovim-tmux-navigator: process already started")
    echo "neovim-tmux-navigator: already started"
    return s:jobId
  endif
endfunction

call s:connect()
