(function() {var implementors = {
"hashbrown":[["impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.Iter.html\" title=\"struct hashbrown::hash_set::Iter\">Iter</a>&lt;'_, K&gt;"],["impl&lt;K, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.Drain.html\" title=\"struct hashbrown::hash_set::Drain\">Drain</a>&lt;'_, K, A&gt;"],["impl&lt;K, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.IntoIter.html\" title=\"struct hashbrown::hash_set::IntoIter\">IntoIter</a>&lt;K, A&gt;"],["impl&lt;K, F, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.ExtractIf.html\" title=\"struct hashbrown::hash_set::ExtractIf\">ExtractIf</a>&lt;'_, K, F, A&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.reference.html\">&amp;K</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.Iter.html\" title=\"struct hashbrown::hash_map::Iter\">Iter</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.IterMut.html\" title=\"struct hashbrown::hash_map::IterMut\">IterMut</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.Keys.html\" title=\"struct hashbrown::hash_map::Keys\">Keys</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.Values.html\" title=\"struct hashbrown::hash_map::Values\">Values</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.ValuesMut.html\" title=\"struct hashbrown::hash_map::ValuesMut\">ValuesMut</a>&lt;'_, K, V&gt;"],["impl&lt;K, V, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.Drain.html\" title=\"struct hashbrown::hash_map::Drain\">Drain</a>&lt;'_, K, V, A&gt;"],["impl&lt;K, V, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.IntoIter.html\" title=\"struct hashbrown::hash_map::IntoIter\">IntoIter</a>&lt;K, V, A&gt;"],["impl&lt;K, V, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.IntoKeys.html\" title=\"struct hashbrown::hash_map::IntoKeys\">IntoKeys</a>&lt;K, V, A&gt;"],["impl&lt;K, V, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.IntoValues.html\" title=\"struct hashbrown::hash_map::IntoValues\">IntoValues</a>&lt;K, V, A&gt;"],["impl&lt;K, V, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_map/struct.ExtractIf.html\" title=\"struct hashbrown::hash_map::ExtractIf\">ExtractIf</a>&lt;'_, K, V, F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.reference.html\">&amp;K</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.reference.html\">&amp;mut V</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_table/struct.Iter.html\" title=\"struct hashbrown::hash_table::Iter\">Iter</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_table/struct.IterMut.html\" title=\"struct hashbrown::hash_table::IterMut\">IterMut</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/raw/struct.RawIter.html\" title=\"struct hashbrown::raw::RawIter\">RawIter</a>&lt;T&gt;"],["impl&lt;T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_table/struct.IntoIter.html\" title=\"struct hashbrown::hash_table::IntoIter\">IntoIter</a>&lt;T, A&gt;<div class=\"where\">where\n    A: Allocator,</div>"],["impl&lt;T, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_table/struct.Drain.html\" title=\"struct hashbrown::hash_table::Drain\">Drain</a>&lt;'_, T, A&gt;"],["impl&lt;T, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/raw/struct.RawDrain.html\" title=\"struct hashbrown::raw::RawDrain\">RawDrain</a>&lt;'_, T, A&gt;"],["impl&lt;T, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/raw/struct.RawIntoIter.html\" title=\"struct hashbrown::raw::RawIntoIter\">RawIntoIter</a>&lt;T, A&gt;"],["impl&lt;T, F, A: Allocator&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_table/struct.ExtractIf.html\" title=\"struct hashbrown::hash_table::ExtractIf\">ExtractIf</a>&lt;'_, T, F, A&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.reference.html\">&amp;mut T</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.Difference.html\" title=\"struct hashbrown::hash_set::Difference\">Difference</a>&lt;'_, T, S, A&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    A: Allocator,</div>"],["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.Intersection.html\" title=\"struct hashbrown::hash_set::Intersection\">Intersection</a>&lt;'_, T, S, A&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    A: Allocator,</div>"],["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.SymmetricDifference.html\" title=\"struct hashbrown::hash_set::SymmetricDifference\">SymmetricDifference</a>&lt;'_, T, S, A&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    A: Allocator,</div>"],["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"hashbrown/hash_set/struct.Union.html\" title=\"struct hashbrown::hash_set::Union\">Union</a>&lt;'_, T, S, A&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    A: Allocator,</div>"]],
"indexmap":[["impl&lt;I, K, V, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Splice.html\" title=\"struct indexmap::map::Splice\">Splice</a>&lt;'_, I, K, V, S&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.tuple.html\">(K, V)</a>&gt;,\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"],["impl&lt;I, T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Splice.html\" title=\"struct indexmap::set::Splice\">Splice</a>&lt;'_, I, T, S&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = T&gt;,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Drain.html\" title=\"struct indexmap::map::Drain\">Drain</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoIter.html\" title=\"struct indexmap::map::IntoIter\">IntoIter</a>&lt;K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoKeys.html\" title=\"struct indexmap::map::IntoKeys\">IntoKeys</a>&lt;K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoValues.html\" title=\"struct indexmap::map::IntoValues\">IntoValues</a>&lt;K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Iter.html\" title=\"struct indexmap::map::Iter\">Iter</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IterMut.html\" title=\"struct indexmap::map::IterMut\">IterMut</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Keys.html\" title=\"struct indexmap::map::Keys\">Keys</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Values.html\" title=\"struct indexmap::map::Values\">Values</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.ValuesMut.html\" title=\"struct indexmap::map::ValuesMut\">ValuesMut</a>&lt;'_, K, V&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Drain.html\" title=\"struct indexmap::set::Drain\">Drain</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.IntoIter.html\" title=\"struct indexmap::set::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Iter.html\" title=\"struct indexmap::set::Iter\">Iter</a>&lt;'_, T&gt;"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Difference.html\" title=\"struct indexmap::set::Difference\">Difference</a>&lt;'_, T, S&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Intersection.html\" title=\"struct indexmap::set::Intersection\">Intersection</a>&lt;'_, T, S&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Union.html\" title=\"struct indexmap::set::Union\">Union</a>&lt;'_, T, S&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"],["impl&lt;T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.SymmetricDifference.html\" title=\"struct indexmap::set::SymmetricDifference\">SymmetricDifference</a>&lt;'_, T, S1, S2&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</div>"]],
"memchr":[["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr.html\" title=\"struct memchr::Memchr\">Memchr</a>&lt;'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr2.html\" title=\"struct memchr::Memchr2\">Memchr2</a>&lt;'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr3.html\" title=\"struct memchr::Memchr3\">Memchr3</a>&lt;'h&gt;"]],
"regex":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIntoIter.html\" title=\"struct regex::bytes::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIntoIter.html\" title=\"struct regex::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIter.html\" title=\"struct regex::bytes::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIter.html\" title=\"struct regex::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"],["impl&lt;'c, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SubCaptureMatches.html\" title=\"struct regex::bytes::SubCaptureMatches\">SubCaptureMatches</a>&lt;'c, 'h&gt;"],["impl&lt;'c, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.SubCaptureMatches.html\" title=\"struct regex::SubCaptureMatches\">SubCaptureMatches</a>&lt;'c, 'h&gt;"],["impl&lt;'r&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.CaptureNames.html\" title=\"struct regex::bytes::CaptureNames\">CaptureNames</a>&lt;'r&gt;"],["impl&lt;'r&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.CaptureNames.html\" title=\"struct regex::CaptureNames\">CaptureNames</a>&lt;'r&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.CaptureMatches.html\" title=\"struct regex::bytes::CaptureMatches\">CaptureMatches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.Matches.html\" title=\"struct regex::bytes::Matches\">Matches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.Split.html\" title=\"struct regex::bytes::Split\">Split</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SplitN.html\" title=\"struct regex::bytes::SplitN\">SplitN</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.CaptureMatches.html\" title=\"struct regex::CaptureMatches\">CaptureMatches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.Matches.html\" title=\"struct regex::Matches\">Matches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.Split.html\" title=\"struct regex::Split\">Split</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex/struct.SplitN.html\" title=\"struct regex::SplitN\">SplitN</a>&lt;'r, 'h&gt;"]],
"regex_automata":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/util/captures/struct.CapturesPatternIter.html\" title=\"struct regex_automata::util::captures::CapturesPatternIter\">CapturesPatternIter</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/util/captures/struct.GroupInfoPatternNames.html\" title=\"struct regex_automata::util::captures::GroupInfoPatternNames\">GroupInfoPatternNames</a>&lt;'a&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/meta/struct.CapturesMatches.html\" title=\"struct regex_automata::meta::CapturesMatches\">CapturesMatches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/meta/struct.FindMatches.html\" title=\"struct regex_automata::meta::FindMatches\">FindMatches</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/meta/struct.Split.html\" title=\"struct regex_automata::meta::Split\">Split</a>&lt;'r, 'h&gt;"],["impl&lt;'r, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_automata/meta/struct.SplitN.html\" title=\"struct regex_automata::meta::SplitN\">SplitN</a>&lt;'r, 'h&gt;"]],
"regex_syntax":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"regex_syntax/utf8/struct.Utf8Sequences.html\" title=\"struct regex_syntax::utf8::Utf8Sequences\">Utf8Sequences</a>"]],
"serde_json":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.IntoIter.html\" title=\"struct serde_json::map::IntoIter\">IntoIter</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Iter.html\" title=\"struct serde_json::map::Iter\">Iter</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.IterMut.html\" title=\"struct serde_json::map::IterMut\">IterMut</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Keys.html\" title=\"struct serde_json::map::Keys\">Keys</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.Values.html\" title=\"struct serde_json::map::Values\">Values</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/map/struct.ValuesMut.html\" title=\"struct serde_json::map::ValuesMut\">ValuesMut</a>&lt;'a&gt;"],["impl&lt;'de, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/iter/traits/marker/trait.FusedIterator.html\" title=\"trait core::iter::traits::marker::FusedIterator\">FusedIterator</a> for <a class=\"struct\" href=\"serde_json/struct.StreamDeserializer.html\" title=\"struct serde_json::StreamDeserializer\">StreamDeserializer</a>&lt;'de, R, T&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"serde_json/de/trait.Read.html\" title=\"trait serde_json::de::Read\">Read</a>&lt;'de&gt; + Fused,\n    T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()