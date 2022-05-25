# fanfare

CLI tool to play random melodies.

### Installation

First you need to [install rust](https://www.rust-lang.org/tools/install).
Then just do:

```bash
git clone https://github.com/Jbat1Jumper/fanfare.git
cargo install --path fanfare
```

### Usage

Play melodies with:

```bash
fanfare play
```

If you liked the last melody, give feedback on it:

```bash
fanfare was-nice
```

After some feedback, focus on making the following melodies be more like the liked ones:

```bash
fanfare focus
```

The intented usage is to use it to notify when a script finishes executing:

```bash
long-long-tests; deploy-awesome-app; sleep 30; fanfare play
```
