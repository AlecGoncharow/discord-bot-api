# discord-bot-api [![Build Status](https://travis-ci.com/AlecGoncharow/discord-bot-api.svg?branch=master)](https://travis-ci.com/AlecGoncharow/discord-bot-api)
This repo provides the structure to provide a web api for one of my other rust projects, a discord bot. Currently has little to no structure, have implemented base64 decoding of [Artifact](https://www.playartifact.com/) Deck Codes.  
[Example](https://aleca-api.herokuapp.com/artifact/decks/decode/ADCJQQGNrgCCJFBGCC7AhAKBRoMCIwGBksKg0FBLQG7AQhPlRIebWVtZXMy)  
Snipped sample:
```json
{
  "cards": [
    {
      "count": 1,
      "id": 10080
    },
    {
      "count": 1,
      "id": 10096
    },
   ],
   "heroes": [
    {
      "id": 10006,
      "turn": 1
    },
    {
      "id": 10014,
      "turn": 1
    },
   ]
```

Which is the same JSON used to generate [this](https://www.playartifact.com/d/ADCJQQGNrgCCJFBGCC7AhAKBRoMCIwGBksKg0FBLQG7AQhPlRIebWVtZXMy)

Now maps decoded Decks to their respective cards:  
[Example](https://aleca-api.herokuapp.com/artifact/decks/deck/ADCJQQGNrgCCJFBGCC7AhAKBRoMCIwGBksKg0FBLQG7AQhPlRIebWVtZXMy)    
Snipped sample:
```json
{
  "cards": [
    {
      "card": {
        "attack": 2,
        "base_card_id": 10080,
        "card_id": 10080,
        "card_name": {
          "brazilian": "Chamariz Rebelde",
          "bulgarian": "Rebel Decoy",
          "czech": "Rebel Decoy"
        },
      }
}
```
