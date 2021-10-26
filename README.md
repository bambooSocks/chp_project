# chp_project

## Running program locally

Using PowerShell
```
cd project
cargo build
Get-Content ..\test_cases\example.txt | cargo run
```

## Run all test cases

If you're using Windows, you can also run `.\run_test_cases.ps1` which will run all test instances from folders `test_cases` and `invalid_test_cases`.