WIP stats app for slippi replays based off of my [stats parser](https://github.com/Walnut356/SlpProcess).

Built using Dioxus and a slightly modified BeerCSS

Special shoutouts to [Waga](https://ssbmtextures.com/other-mod-types/modernized-stock-icons/) for the stock icons

Microsoft Defender false positive
---

Microsoft Defender on my laptop detects "Trojan:Win32/Wacatac.B!ml". Apparently this is an over-eager machine learning algorithm doing the detecting. See: [this windows help forum.](https://answers.microsoft.com/en-us/windows/forum/all/overly-eager-heuristics-for-trojanwin32wacatacbml/6f2a72f3-3978-48ac-9fb7-fbe82c686ae3). Wacatac appears to be a pretty common false positive, especially with Rust binaries - a simple google search for "Rust wacatac reddit" reveals quite a few hits. You can restore the files via Windows Security -> Virus & Threat Protection -> Protection History
