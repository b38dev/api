# API for Bangumi

url: `https://api.b38.dev`

## V1

* nest path `v1/`
* Error Response
```json
{
    "error": "error message",
    "status": 500
}
```


### OnAir

-   Path `v1/onair`
-   Method `GET`
-   Query `?subjects=[subject_id][,subject_id]`
-   Response example with subjects `512190,515880`

```json
{ "data": [
    [ 512190, {
        "title": "瑠璃の宝石",
        "titleTranslate": { "zh-Hans": ["琉璃的宝石"], "en": ["Ruri Rocks"], "zh-Hant": ["琉璃的寶石"] },
        "type": "tv",
        "lang": "ja",
        "officialSite": "https://rurinohouseki.com/",
        "begin": "2025-07-06T12:00:00.000Z",
        "broadcast": "R/2025-07-06T12:00:00.000Z/P7D",
        "end": "2025-09-28T12:30:00.000Z",
        "comment": "",
        "sites": [
            { "site": "bangumi", "id": "512190" },
            { "site": "unext", "id": "SID0200161", "begin": "2025-07-09T03:00:00.000Z", "broadcast": "R/2025-07-09T03:00:00.000Z/P7D" },
            { "site": "nicovideo", "id": "rurinohouseki", "begin": "", "broadcast": "" },
            { "site": "abema", "id": "26-249", "begin": "", "broadcast": "" },
            { "site": "bangumi_moe", "id": "6862b060dffbea3b3f911b03" },
            { "site": "bilibili", "id": "26624922", "begin": "2025-07-31T10:00:00.000Z", "broadcast": "R/2025-07-31T10:00:00.000Z/P7D" },
            { "site": "crunchyroll", "id": "G4PH0WJ2V", "begin": "2025-07-06T12:00:00.000Z", "broadcast": "R/2025-07-06T12:00:00.000Z/P7D" },
            { "site": "gamer", "id": "141362", "begin": "2025-07-06T13:30:00.000Z", "broadcast": "R/2025-07-06T13:30:00.000Z/P7D" },
            { "site": "gamer_hk", "id": "141362", "begin": "2025-07-06T13:30:00.000Z", "broadcast": "R/2025-07-06T13:30:00.000Z/P7D" },
            { "site": "danime", "id": "28061", "begin": "2025-07-09T03:00:00.000Z", "broadcast": "R/2025-07-09T03:00:00.000Z/P7D" },
            { "site": "prime", "id": "B0DZ6JGQPL", "begin": "", "broadcast": "" },
            { "site": "mikan", "id": "3663" }
        ]
    } ],
    [ 515880, {
        "title": "ぐらんぶる Season 2",
        "titleTranslate": { "zh-Hans": ["碧蓝之海 第二季"], "zh-Hant": ["GRAND BLUE 碧藍之海 2", "GRANDBLUE 碧藍之海2"] },
        "type": "tv",
        "lang": "ja",
        "officialSite": "https://grandblue-anime.com/",
        "begin": "2025-07-07T15:30:00.000Z",
        "broadcast": "R/2025-07-07T15:30:00.000Z/P7D",
        "end": "2025-09-22T15:30:00.000Z",
        "comment": "",
        "sites": [
            { "site": "bangumi", "id": "515880" },
            { "site": "nicovideo", "id": "grandblue2-anime", "begin": "", "broadcast": "" },
            { "site": "unext", "id": "SID0200153", "begin": "2025-07-07T16:00:00.000Z", "broadcast": "R/2025-07-07T16:00:00.000Z/P7D" },
            { "site": "abema", "id": "11-72", "begin": "", "broadcast": "" },
            { "site": "bangumi_moe", "id": "6862b065dffbea3b3f911b1b" },
            { "site": "mikan", "id": "3661" },
            { "site": "gamer_hk", "id": "141653", "begin": "2025-07-07T16:30:00.000Z", "broadcast": "R/2025-07-07T16:30:00.000Z/P7D" },
            { "site": "gamer", "id": "141653", "begin": "2025-07-07T16:30:00.000Z", "broadcast": "R/2025-07-07T16:30:00.000Z/P7D" },
            { "site": "tropics", "id": "PLoGVqODys23Bsny5PRn53AdGkUxlk8jpf", "begin": "2025-07-07T16:30:06.000Z", "broadcast": "R/2025-07-07T16:30:06.000Z/P7D" },
            { "site": "prime", "id": "B0DBMTXW3G", "begin": "", "broadcast": "" },
            { "site": "danime", "id": "28133", "begin": "2025-07-12T16:00:00.000Z", "broadcast": "R/2025-07-12T16:00:00.000Z/P7D" },
            { "site": "bilibili", "id": "26714035", "begin": "2025-07-22T12:30:00.000Z", "broadcast": "R/2025-07-22T12:30:00.000Z/P7D" }
        ]
    } ]
] }
```

### User Name History

-   Path `v1/user/name-history`
-   Method `GET`
-   Query `?uid=[uid]`
-   Response example with uid `sai`

```json
{ "data": {
    "state": "active",
    "nid": 1,
    "sid": "sai",
    "name_history": {
        "update_at": "2025-10-10T10:47:24.819977961Z",
        "key_point": "2025-10-2",
        "names": ["ıɐs", "Sai🖖", "Sai😊", "Sai", "Sai 😊"]
    }
} }
```

-   Response example with uid `sai`, first time

```json
{ "data": {
    "state": "active",
    "nid": 1,
    "sid": "sai",
    "name_history": null
} }
```
