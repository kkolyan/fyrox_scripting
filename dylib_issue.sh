set -e
RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p agent
RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p invoker

rm -rf dylib_issue/lab
mkdir dylib_issue/lab

cp target/debug/agent.dll dylib_issue/lab
cp target/debug/invoker.exe dylib_issue/lab
./get_std_dll.sh dylib_issue/lab
cp target/debug/deps/fyrox_dylib-*.dll dylib_issue/lab

cd dylib_issue/lab

./invoker.exe