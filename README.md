# Hebro Rust (basic rewrite of the c++ version of Hebro in Rust)

This repository contains a memory exploit code targeting the explorer.exe process on Windows systems. The exploit aims to reserve virtual memory regions in the target process, which can lead to system instability and the need for a full system restart.

### How It Works

The exploit leverages the `VirtualAllocEx` function from the Windows API to reserve virtual memory regions in the explorer.exe process. It searches for free memory regions within a specified address range and allocates memory by calling `VirtualAllocEx` with the `MEM_RESERVE` flag. The exploit utilizes the `VirtualQueryEx` function to query the memory information of the process.

The code repeatedly performs the memory reservation operation within a specified address range. The exploit aligns the memory addresses, queries the memory information using `VirtualQueryEx`, and reserves the memory using `VirtualAllocEx`. This process continues until the entire address range has been scanned or until an error occurs.

+++

Tämä repo sisältää muistin hyödyntämiskoodin, joka kohdistuu explorer.exe-prosessiin Windows-järjestelmissä. Hyökkäyksen yhteydessä varataan virtuaalimuistialueita kohdeprosessissa, mikä voi johtaa järjestelmän epävakaisuuteen ja jolloin saattaa olla tarpeen suorittaa täysi järjestelmän uudelleenkäynnistys.

### Toimintaperiaate

Hyökkäys hyödyntää VirtualAllocEx-funktiota Windows API:sta varatakseen virtuaalimuistialueita explorer.exe-prosessissa. Se etsii vapaata muistialuetta määritellyltä osoitealueelta ja varaa muistia kutsuen VirtualAllocEx-funktiota MEM_RESERVE-lipun kanssa. Hyökkäys käyttää VirtualQueryEx-funktiota prosessin muistitietojen tiedustelemiseen.

Koodi suorittaa toistuvasti muistinvarausoperaation määritellyllä osoitealueella. Hyökkäys tasaa muistiosoitteet, tiedustelee muistitietoja VirtualQueryEx-funktiolla ja varaa muistin VirtualAllocEx-funktiolla. Tämä prosessi jatkuu, kunnes koko osoitealue on käyty läpi tai tapahtuu virhe.