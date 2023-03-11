call c:\tools\VisualStudio2019\Community\VC\Auxiliary\Build\vcvars32.bat

rem C:\tools\VisualStudio2019\Community\VC\Tools\MSVC\14.24.28314\ATLMFC\include;^
rem C:\tools\VisualStudio2019\Community\VC\Tools\MSVC\14.24.28314\include;^


set INCLUDE=%INCLUDE%;^
c:\tools\WindowsKits\10\Include\10.0.19041.0\um;^
c:\tools\WindowsKits\10\Include\10.0.19041.0\shared;^
c:\tools\WindowsKits\10\Include\10.0.19041.0\ucrt

set LIB=%LIB%;^
C:\tools\VisualStudio2019\Community\VC\Tools\MSVC\14.24.28314\ATLMFC\lib\x86;^
C:\tools\VisualStudio2019\Community\VC\Tools\MSVC\14.24.28314\lib\x86;^
c:\tools\WindowsKits\10\Lib\10.0.19041.0\um\x86;^
c:\tools\WindowsKits\10\Lib\10.0.19041.0\ucrt\x86;

rem ***
rem *** for msbuild. hope so
rem ***
set IncludePath=%INCLUDE%