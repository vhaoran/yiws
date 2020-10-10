rm -rf *.out
echo "making....."
echo $(pwd)

CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl

cp ./target/release/yi-ws  ./yi-ws.out
# upx -9 *.out

scp  -i ~/.ssh/yh.pem *.out  root@yh:~/prog-yi/ws/
scp  -i ~/.ssh/yh.pem *.toml  root@yh:~/prog-yi/ws/

# rm -rf *.out
echo " ok "


