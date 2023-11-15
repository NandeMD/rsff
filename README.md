# rsff

rsff` (scanlation file format) is the core library of an application designed to facilitate the work of teams translating content such as manga, manhwa, manhua, webtoons, etc.

## Some Examples:

```rust
use rsff::Document;
use rsff::balloon::Balloon;

// Create a default document.
let mut d: Document = Document::default();

// Create a default balloon.
let mut b: Balloon = Balloon::default();

// Add content to the balloon.
b.tl_content.push("This is a translation line.".to_string());

// Add balloon to the document.
d.balloons.push(b);
```

## Basic Raw SFF XML File:

```xml
<Document>
    <Metadata>
        <Script>Scanlation Script File v0.2.0</Script>
        <App></App>
        <Info>Num</Info>
        <TLLength>9</TLLength>
        <PRLength>6</PRLength>
        <CMLength>0</CMLength>
        <BalloonCount>2</BalloonCount>
        <LineCount>2</LineCount>
    </Metadata>
    <Balloons>
        <Balloon type="OT">
            <TL>num</TL>
            <TL>nam</TL>
            <PR>numnam</PR>
        </Balloon>
        <Balloon type="Dialogue">
            <TL>num</TL>
        </Balloon>
    </Balloons>
</Document>
```
