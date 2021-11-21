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
let s:command = ['/Users/amiel/src/neovim-tmux-navigator/target/debug/neovim-tmux-navigator', 'server']

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

function! s:configureCommands()
  command! NvimTmuxNavigatorUp :call rpcnotify(s:jobId, s:Up)
  command! NvimTmuxNavigatorDown :call rpcnotify(s:jobId, s:Down)
  command! NvimTmuxNavigatorLeft :call rpcnotify(s:jobId, s:Left)
  command! NvimTmuxNavigatorRight :call rpcnotify(s:jobId, s:Right)
endfunction

" Initialize RPC
function! s:initRpc()
  if s:jobId == 0
    let jobid = jobstart(s:command, { 'rpc': v:true })
    echo "neovim-tmux-navigator: started rpc process"
    return jobid
  else
    echo "neovim-tmux-navigator: already started"
    s:initRpc
    return s:jobId
  endif
endfunction

call s:connect()
