@echo off
for /f "tokens=*" %%a in ('zebar monitors') do (
    start /b zebar open bar --args %%a
)
