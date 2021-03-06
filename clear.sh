#/bin/bash
for f in *;  do 
    if [ -d $f  -a ! -h $f ];  
    then  
        cd -- "$f";  
        echo "Doing something in folder `pwd`/$f"; 
        cargo clean

        cd ..; 
    fi;  
done; 
