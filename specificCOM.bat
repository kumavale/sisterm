@echo off
ECHO Processing...
FOR /L %%a in (1,1,100) do ( MODE COM%%a /STATUS | FINDSTR "COM.*:$" )
ECHO.
ECHO Complete!
SET /P _="Press any key..."
