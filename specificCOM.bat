@echo off
FOR /L %%a in (1,1,9) do ( MODE COM%%a /STATUS | FINDSTR "COM.*:$" )
