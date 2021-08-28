# `ogam`

## What is `ogam`?

Typographic rules can be trick to enforce in a consistent manner in a
document. In French, typographic rules for *dialogues* are arguably
worst.  `ogam` is a domain-specific markup language, intended to free
fiction writers from the pain to enforce typographic rules within
their productions manually.

Additionally, `ogam` allows to annotate one’s text with metadata.  For
instance, it is possible to indicate what character is speaking a
given line of dialogue.  This paves the road towards being able to
discoverability, *e.g.*, finding the list of scenes wherein a given
character speaks.

## The Language

### Formatting

```ogam
This is a regular sentence.
```

```html
<p>This is a regular sentence.</p>
```

> <p>This is a regular sentence.</p>

------------------------

```ogam
*This is a sentence with emphasize.*</p>
```

```html
<p><em>This is a regular sentence.</em></p>
```

> <p><em>This is a regular sentence.</em></p>

------------------------

```ogam
+This is a sentence with strong emphasize.+</p>
```

```html
<p><strong>This is a sentence with strong emphasize.</strong></p>
```

> <p><strong>This is a sentence with strong emphasize.</strong></p>

------------------------

```ogam
This +is+ a *sentence with +multiple emphasizes+*.
```

```html
<p>This <strong>is</strong> a <em>sentence with <strong>multiple emphasizes</strong></em>.</p>
```

> <p>This <strong>is</strong> a <em>sentence with <strong>multiple
> emphasizes</strong></em>.</p>

------------------------

```ogam
"This is a +quote+."
```

```html
<p>“This is a <strong>quote</strong>.”</p>
```

> <p>“This is a <strong>quote</strong>.”</p>

------------------------

```ogam
This +is an "incorrect+" sentence.
```

```html
<p>This <span class="illformed_inline">+is an "incorrect+" sentence.</span></p>
```

------------------------

```ogam
This +is an "incorrect+"
sentence.
```

```html
<p>This <span class="illformed_inline">+is an "incorrect+"</span> sentence.</p>
```

### Dialogues

```ogam
[Hi,| she says.|](Clara)
```

```html
<p><span class="dialogue by-Clara">“<span class="reply">Hi,</span>” she says.</span></p>
```

> <p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says.</span></p>

------------------------

```ogam
[Hi,| she says.| How are you?](Clara)
```

```html
<p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span></p>
```

> <p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span></p>

------------------------

```ogam
[Hi,| she says.| How are you?](Clara) [I'm fine, thanks.]
```

```html
<p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span></p><p><span class="dialogue">“<span class="reply">I’m fine, thanks.</span>”</span></p>
```

> <p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span></p><p><span class="dialogue">“<span class="reply">I’m fine, thanks.</span>”</span></p>

------------------------

```ogam
[Hi,| she says.| How are you?](Clara) She was smiling. [I'm fine, thanks.]
```

```html
<p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span> She was smiling. <span class="dialogue">“<span class="reply">I’m fine, thanks.</span>”</span></p>
```

> <p><span class="dialogue by-Clara">“<span class="reply">Hi,</span> she says. <span class="reply">How are you?</span>”</span> She was smiling. <span class="dialogue">“<span class="reply">I’m fine, thanks.</span>”</span></p>

------------------------

```ogam
It was a hot day. [Hi!| he said.](Aaron)
His friends smiled.
```

```html
<div><p>It was a hot day.<span class="illformed_inline">[Hi!| he said.](Aaron)</span> His friends smiled. </p></div>
```

> <div><p>It was a hot day.<span class="illformed_inline">[Hi!| he said.](Aaron)</span> His friends smiled. </p></div>

### Paragraphs


```ogam
This is a first paragraph.

This is a second paragraph.
```

```html
<div><p>This is a first paragraph.</p><p>This is a second paragraph</p>
```

> <div><p>This is a first paragraph.</p><p>This is a second paragraph</p>

------------------------

```ogam
[Ceci est une première ligne de dialogue.]

[Ceci est une autre ligne de dialogue.]
```

```html
<div><p><span class="dialogue">«&nbsp;<span class="reply">Ceci est une première ligne de dialogue.</span></span></p><p><span class="dialogue">— <span class="reply">Ceci est une autre ligne de dialogue.</span></span></p></div>
```

> <div><p><span class="dialogue">«&nbsp;<span class="reply">Ceci est une première ligne de dialogue.</span></span></p><p><span class="dialogue">— <span class="reply">Ceci est une autre ligne de dialogue.</span></span></p></div>

## The Implementations

A first implementation of an `ogam` —`ogmarkup` at this time– parser
and compiler has been written in Haskell, and [has been published on
hackage (MIT)](https://hackage.haskell.org/package/ogmarkup).

This first project has been deprecated in favor of this rewriting in
Rust.  Contrary to its predecessor, `ogam` is released under the terms
of the MPL 2.0.
