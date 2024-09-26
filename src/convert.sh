ffmpeg -i blaziken.gif -vf scale=96:64 blaziken.gif;
mkdir temp;
ffmpeg -i blaziken.gif -vsync 0 temp/temp%d.png;

for ((i = 1; i <= 86; i++));
do
	cargo run temp/temp$i.png
done


