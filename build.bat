@echo off

cls

for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "date=%dt:~0,4%_%dt:~4,2%_%dt:~6,2%"

set "name=syzygy"

cargo build --target-dir G:\%name%\target\%date%\ --examples

xcopy "src\"     "target\%date%\debug\src\"  /v /q /s /e /y > nul