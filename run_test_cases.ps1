cd project

Write-Host "Running invalid tests"
Get-Content ..\invalid_test_cases\empty.swe | cargo run
Get-Content ..\invalid_test_cases\only_k.swe | cargo run
Get-Content ..\invalid_test_cases\k_is_not_number.swe | cargo run
Get-Content ..\invalid_test_cases\only_s.swe | cargo run
Get-Content ..\invalid_test_cases\less_ts_than_k_eof.swe | cargo run
Get-Content ..\invalid_test_cases\less_ts_than_k.swe | cargo run
Get-Content ..\invalid_test_cases\invalid_s.swe | cargo run
Get-Content ..\invalid_test_cases\more_ts_than_k.swe | cargo run
Get-Content ..\invalid_test_cases\no_expansions.swe | cargo run
Get-Content ..\invalid_test_cases\empty_ri.swe | cargo run
Get-Content ..\invalid_test_cases\invalid_ri.swe | cargo run
Get-Content ..\invalid_test_cases\invalid_ti.swe | cargo run

Write-Host "Running valid tests"
Get-Content ..\test_cases\test01.swe | cargo run 
Get-Content ..\test_cases\test02.swe | cargo run 
Get-Content ..\test_cases\test03.swe | cargo run 
Get-Content ..\test_cases\test04.swe | cargo run 
Get-Content ..\test_cases\test05.swe | cargo run 
Get-Content ..\test_cases\test06.swe | cargo run 

cd ..