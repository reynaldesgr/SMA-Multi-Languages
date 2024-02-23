FILE="output.txt"

#echo "Run C++..."
#cd "cpp"
#echo "C++" >> $FILE
#g++ -I./include -o main ./src/*.cpp
#time ./main >> $FILE
#echo "Done."

echo "Run Java..."
cd "java"
echo "Java" >> $FILE
javac src/*.java
time java -cp src Main >> $FILE
echo "Done."

echo "Run Python..."
cd "../python"
echo "Python" >> $FILE
time python3 src/main.py >> $FILE
echo "Done."

echo "Run Ruby..."
cd "../ruby/src"
echo "Ruby" >> $FILE
time ruby main.rb 
echo "Done."

echo "Run Rust..."
cd "../../rust"
echo "Rust" >> $FILE
cargo build --release
time cargo run >> $FILE
echo "Done."