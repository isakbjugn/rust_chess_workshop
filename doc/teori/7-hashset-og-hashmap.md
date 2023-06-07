# 7 – HashSet og HashMap

<span style="justify-content: space-between; display: flex"><span>
    [← 6 Vektor og iterator](./6-vektor-og-iterator.md)
</span> <span>
    [8 clone og copy →](./8-clone-og-copy.md)
</span></span>

<span class="header"><span></span></span>

___

## HashSet
En matematisk mengde, som i Rust er implementert som `HashSet`, kjennetegnes ved at elementenes rekkefølge ikke har
betydning, og at ingen elementer opptrer flere ganger. I metodene som beregner lovlige trekk for en brikke er det ofte
nyttig å bruke metoder knyttet til `HashSet`, blant annet:

* `HashSet::union`: Gir alle verdiene som finnes i to `HashSet`

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_union: HashSet<_> = set_1.union(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2, 3, 4, 5]))
```

* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet

Brukes på følgende måte
```rust
let set_1 = HashSet::from_iter([1, 2, 3]);
let set_2 = HashSet::from_iter([3, 4, 5]);
let set_difference: HashSet<_> = set_1.difference(&set_2).collect()
assert_eq(set_union, HashSet::from_iter([1, 2]))
```

I tillegg har du
* `HashSet::difference`: Gir alle verdiene som er unike for ett `HashSet` sammenliknet med et annet
* `HashSet::symmetric_difference`: Gir alle verdiene som er unike for to `HashSet` og *ikke* finnes i begge

Les mer om `HashSet` og lær hvordan disse metodene brukes i [Rust-dokumentasjonen om `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
