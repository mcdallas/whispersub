
## whispersub

A dead simple utility to format the output of OpenAI's whisper model (or whisper.cpp) into an .srt file.


### Usage

``` bash
whispersub input.txt -o output.srt
```

you can also pipe the output of whisper.cpp into whispersub


```bash
whisper-cpp --file audio.wav --language en --model ggml-medium.en.bin | whispersub
```

or use a little hellper function to extract the audio from a video, pipe it to whisper.cpp and then to whispersub

```bash
makesub () {
    filename=$(basename -- "$1")
    filename="${filename%.*}"
    model=${HOME}/.local/share/whisper/ggml-medium.en.bin
    ffmpeg -i "$1" -vn -acodec pcm_s16le -ar 16000 -ac 2 -f wav - | 
    nice -n 20 whisper-cpp --threads "$(nproc)" --file - --language en --model "$model" |
    whispersub -o "${filename}.en.srt"
}

makesub video.mp4
```