" pass --package and name for to run tests from a single module
command! -nargs=1 Test Terminal cargo test
map <F5> :Test %<CR>

command! Build Terminal cargo -q run
map <F6> :Build<CR>
