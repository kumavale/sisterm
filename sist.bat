@echo off
TITLE sisterm
FOR /L %%a in (1,1,20) do ( MODE COM%%a /STATUS | FINDSTR "COM.*:$" )
echo.
SET /P COM="COM port number: "
wsl sudo chmod 666 /dev/ttyS%COM%;^
  if [ $? -eq 0 ];then sist -l /dev/ttyS%COM%; fi

if not %ERRORLEVEL% == 0 (
  echo.
  SET /P _="Press any key..."
  exit /b 1
)
