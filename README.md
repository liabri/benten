# benten (WIP)
A flexible input method engine made and tested only for unix systems, which allows the easy configuration of mixed-script writing systems such as Kana/Kanji & Hangul/Hanja.

## motivation
As I started to learn Japanese, I wanted a way to input without having to rely on candidate windows and such, which led me to find Cangjie and other shaped based input method for chinese characters. This idea was to utilise a kana layout with cangjie hidden away behind a single key press, this could've easily been done in Rime or just using a keyboard remapper, but I thought it'd be a fun project and would satisfy my desire of not jerry rigging appplications together. This has led to the birth of `benten`, a decently opinionated but flexible input method engine.

## why the name `benten` ?
Named after the Japanese Buddhist Goddess "Benzaiten" (弁才天) whom stands for all things that flow (as per Wikipedia), which hopefully represents this project well :).  

## todo
- BentenCli;
- Unicode Method;
- How are differrent glyphs of many CJK characters handled ?
- Possibly abstract key codes;
- Possibly refactor `Layout` & `LayoutMethod`;
- Prevent recreation of `BaseDirectories` struct in deserialisation methods in mode/parser.rs;
- BTreeMaps ?

## configuration
As of now all the configuration is done in `$XDG_CONFIG_HOME`, consisting of three folders: 
1. `layouts`: key map and layout configuration, defined in `*.layout.yaml`;
2. `tables`: complementary table method for handling table-based lookup input methods, defined in `*.dict.yaml`;
3. `modes`: the glue holding the pieces together, contains global hot keys, relationships between different methods, defined in `*mode.yaml`.
