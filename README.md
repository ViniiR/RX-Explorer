# RX Explorer

A Fast File Explorer written in the Rust programming language, but how fast do i mean? good question on my personal measurements it was able to iterate over all of my C Drive in about 22 seconds! for comparison windows explorer found the first file in 1:35~ seconds and by first file it also means it was not able to iterate over all of my C drive which is about 158 GiB / 237GiB on the writing of this file.

![RX-Explorer logo](./src/assets/RXE-logo.png)
This is the official logo for RX Explorer

update a few days later: the program is not as fast as early mentioned, it has been able to run over all of my C drive in about 1:30 minutes but after improvement i* was able to decrease it to about 40 seconds, overall i do not recommend using it on the root directory, it is more efficient in directories under the local user.

This file explorer is by no means an actual and stable product, it has been developed by the soul purpose of personal use and learning new technologies such as vue, tailwindcss and tauri.

Important notes for anyone who wants to download it: this program has been developed only with windows 10 and single hard drive Desktops in mind. it may work with windows 10 or 11 multi hard drive Desktops but there will be bugs

I do not have any affiliation with the Rust Foundation.

All icons credits go to [Freepik](https://br.freepik.com).
