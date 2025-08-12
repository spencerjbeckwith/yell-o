# yell-o

Alternatively, the annoy-the-shit-outta-Kevin machine. This is an application to run on your PC (or Raspberry PI) that'll allow you to send voice AI messages. As long as they have internet access and speakers (and the application installed, duh) you'll get to hear whatever nonsense everybody wants to say to you, without the exhausting physical activity of walking to the other room.

Each running application is a Rust app that serves a static React frontend. This frontend makes it easy to submit text, which is sent to the API, converted into AI sound data, then played on the host machine.

## Installation/Configuration

### Binary

You may download a pre-built yell-o binary from [releases](https://github.com/spencerjbeckwith/yell-o/releases) according to your architecture (amd64 or arm64).

When running the binary, you will either need a `.env` file in the same directory as the executable, or set the environment in your command-line.

### .deb

The .deb package is useful for an easy system-wide installation, such as for setting up a Raspberry Pi in another room with a speaker connected. This installation includes a systemd daemon, so no further steps are required to keep it "always-on". You may download an installable .deb package from [releases](https://github.com/spencerjbeckwith/yell-o/releases) according to your architecture (amd64 or arm64).

This package uses a system-wide installation of pulseaudio, which [is strongly discouraged.](https://www.freedesktop.org/wiki/Software/PulseAudio/Documentation/User/SystemWide/). However, this is the only way to allow sound to play without having a user actively logged in. Therefore, it is not a good idea to install the .deb on any desktop system that is regularly used. pulseaudio must also be installed before the .deb.

```bash
sudo apt-get install pulseaudio
sudo dpkg -i yell-o_<version>-arm64.deb
```

After installation, you need to modify `/etc/yell-o/yell-o.env` to include the correct values for your environment variables. Restart the service afterwards:

```bash
sudo systemctl restart yell-o
```

The final, and most important part, of setting up the application will be to install on every Raspberry PI you own, assign static IP addresses to them all, then print out enormous QR codes to frame on your walls. For best results, tell your roommates to bookmark the IP and port. Just be aware that spamming too much will eventually run you out of ElevenLabs tokens.

### Required Variables

- `ELEVENLABS_API_KEY`

### Optional Variables

- `HOST`: Hostname to use for the Flask server.
- `PORT`: Port to use for the Flask server.

## Development

There is a [bruno](https://www.usebruno.com/) collection under the `/bruno` directory, which can be used to easily test the API.

To build the app package, you may run the `./package.sh`. This will attempt to build for both x86_64 and aarch64, so it assumes your Rust toolchains for both are configured, and that you have the appropriate development libraries installed on your system.
