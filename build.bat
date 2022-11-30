@echo off
cd java
rmdir /S /Q build
rmdir /S /Q javadoc
cd src
dir /s /B *.java > sources.txt
"C:\Java\jdk-11.0.12\bin\javac.exe" -d ../build @sources.txt
echo java build finished. Generating javadoc...
"C:\Java\jdk-11.0.12\bin\javadoc.exe" -author -public -d ../javadoc -tag "apiNote:a:API Note:" -tag "implNote:a:Implementation Note:" @sources.txt
del sources.txt
echo generationg javadoc finished. Packing...
cd ../build
@rem dir /s /B *.class > classes.txt
"C:\Java\jdk-11.0.12\bin\jar.exe" cf Ev3Lib.jar -C .. javadoc -C ../resources libev3.so . 
@rem del classes.txt
echo Packing finished. Done.