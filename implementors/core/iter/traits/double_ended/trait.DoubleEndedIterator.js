(function() {var implementors = {
"allocator_api2":[["impl&lt;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"allocator_api2/vec/struct.Splice.html\" title=\"struct allocator_api2::vec::Splice\">Splice</a>&lt;'_, I, A&gt;"],["impl&lt;T, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"allocator_api2/vec/struct.Drain.html\" title=\"struct allocator_api2::vec::Drain\">Drain</a>&lt;'_, T, A&gt;"],["impl&lt;T, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"allocator_api2/vec/struct.IntoIter.html\" title=\"struct allocator_api2::vec::IntoIter\">IntoIter</a>&lt;T, A&gt;"],["impl&lt;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"allocator_api2/boxed/struct.Box.html\" title=\"struct allocator_api2::boxed::Box\">Box</a>&lt;I, A&gt;"]],
"arrayvec":[["impl&lt;'a, T: 'a, const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"arrayvec/struct.Drain.html\" title=\"struct arrayvec::Drain\">Drain</a>&lt;'a, T, CAP&gt;"],["impl&lt;T, const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"arrayvec/struct.IntoIter.html\" title=\"struct arrayvec::IntoIter\">IntoIter</a>&lt;T, CAP&gt;"]],
"bit_vec":[["impl&lt;B: <a class=\"trait\" href=\"bit_vec/trait.BitBlock.html\" title=\"trait bit_vec::BitBlock\">BitBlock</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bit_vec/struct.IntoIter.html\" title=\"struct bit_vec::IntoIter\">IntoIter</a>&lt;B&gt;"],["impl&lt;'a, B: <a class=\"trait\" href=\"bit_vec/trait.BitBlock.html\" title=\"trait bit_vec::BitBlock\">BitBlock</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bit_vec/struct.Blocks.html\" title=\"struct bit_vec::Blocks\">Blocks</a>&lt;'a, B&gt;"],["impl&lt;'a, B: <a class=\"trait\" href=\"bit_vec/trait.BitBlock.html\" title=\"trait bit_vec::BitBlock\">BitBlock</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bit_vec/struct.Iter.html\" title=\"struct bit_vec::Iter\">Iter</a>&lt;'a, B&gt;"]],
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = L::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</span>"]],
"generic_array":[["impl&lt;T, N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"generic_array/iter/struct.GenericArrayIter.html\" title=\"struct generic_array::iter::GenericArrayIter\">GenericArrayIter</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    N: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;T&gt;,</span>"]],
"image":[["impl&lt;'a, P: <a class=\"trait\" href=\"image/trait.Pixel.html\" title=\"trait image::Pixel\">Pixel</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"image/buffer/struct.Pixels.html\" title=\"struct image::buffer::Pixels\">Pixels</a>&lt;'a, P&gt;<span class=\"where fmt-newline\">where\n    P::<a class=\"associatedtype\" href=\"image/trait.Pixel.html#associatedtype.Subpixel\" title=\"type image::Pixel::Subpixel\">Subpixel</a>: 'a,</span>"],["impl&lt;'a, P: <a class=\"trait\" href=\"image/trait.Pixel.html\" title=\"trait image::Pixel\">Pixel</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"image/buffer/struct.RowsMut.html\" title=\"struct image::buffer::RowsMut\">RowsMut</a>&lt;'a, P&gt;<span class=\"where fmt-newline\">where\n    P::<a class=\"associatedtype\" href=\"image/trait.Pixel.html#associatedtype.Subpixel\" title=\"type image::Pixel::Subpixel\">Subpixel</a>: 'a,</span>"],["impl&lt;'a, P: <a class=\"trait\" href=\"image/trait.Pixel.html\" title=\"trait image::Pixel\">Pixel</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"image/buffer/struct.Rows.html\" title=\"struct image::buffer::Rows\">Rows</a>&lt;'a, P&gt;<span class=\"where fmt-newline\">where\n    P::<a class=\"associatedtype\" href=\"image/trait.Pixel.html#associatedtype.Subpixel\" title=\"type image::Pixel::Subpixel\">Subpixel</a>: 'a,</span>"],["impl&lt;'a, P: <a class=\"trait\" href=\"image/trait.Pixel.html\" title=\"trait image::Pixel\">Pixel</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"image/buffer/struct.PixelsMut.html\" title=\"struct image::buffer::PixelsMut\">PixelsMut</a>&lt;'a, P&gt;<span class=\"where fmt-newline\">where\n    P::<a class=\"associatedtype\" href=\"image/trait.Pixel.html#associatedtype.Subpixel\" title=\"type image::Pixel::Subpixel\">Subpixel</a>: 'a,</span>"]],
"indexmap":[["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoKeys.html\" title=\"struct indexmap::map::IntoKeys\">IntoKeys</a>&lt;K, V&gt;"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Difference.html\" title=\"struct indexmap::set::Difference\">Difference</a>&lt;'_, T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IterMut.html\" title=\"struct indexmap::map::IterMut\">IterMut</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Keys.html\" title=\"struct indexmap::map::Keys\">Keys</a>&lt;'_, K, V&gt;"],["impl&lt;T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.SymmetricDifference.html\" title=\"struct indexmap::set::SymmetricDifference\">SymmetricDifference</a>&lt;'_, T, S1, S2&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoIter.html\" title=\"struct indexmap::map::IntoIter\">IntoIter</a>&lt;K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Drain.html\" title=\"struct indexmap::map::Drain\">Drain</a>&lt;'_, K, V&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.IntoValues.html\" title=\"struct indexmap::map::IntoValues\">IntoValues</a>&lt;K, V&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Drain.html\" title=\"struct indexmap::set::Drain\">Drain</a>&lt;'_, T&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.ValuesMut.html\" title=\"struct indexmap::map::ValuesMut\">ValuesMut</a>&lt;'_, K, V&gt;"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Union.html\" title=\"struct indexmap::set::Union\">Union</a>&lt;'_, T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Values.html\" title=\"struct indexmap::map::Values\">Values</a>&lt;'_, K, V&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.IntoIter.html\" title=\"struct indexmap::set::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/map/struct.Iter.html\" title=\"struct indexmap::map::Iter\">Iter</a>&lt;'_, K, V&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Iter.html\" title=\"struct indexmap::set::Iter\">Iter</a>&lt;'_, T&gt;"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"indexmap/set/struct.Intersection.html\" title=\"struct indexmap::set::Intersection\">Intersection</a>&lt;'_, T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"]],
"memchr":[["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr3.html\" title=\"struct memchr::Memchr3\">Memchr3</a>&lt;'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::all::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr.html\" title=\"struct memchr::Memchr\">Memchr</a>&lt;'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr2.html\" title=\"struct memchr::Memchr2\">Memchr2</a>&lt;'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::all::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.OneIter.html\" title=\"struct memchr::arch::all::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"]],
"nalgebra":[["impl&lt;'a, T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S: 'a + <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorageMut.html\" title=\"trait nalgebra::base::storage::RawStorageMut\">RawStorageMut</a>&lt;T, R, C&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"nalgebra/base/iter/struct.ColumnIterMut.html\" title=\"struct nalgebra::base::iter::ColumnIterMut\">ColumnIterMut</a>&lt;'a, T, R, C, S&gt;"],["impl&lt;'a, T, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S: 'a + <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"nalgebra/base/iter/struct.ColumnIter.html\" title=\"struct nalgebra::base::iter::ColumnIter\">ColumnIter</a>&lt;'a, T, R, C, S&gt;"],["impl&lt;'a, T, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S: 'a + <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"nalgebra/base/iter/struct.MatrixIter.html\" title=\"struct nalgebra::base::iter::MatrixIter\">MatrixIter</a>&lt;'a, T, R, C, S&gt;"],["impl&lt;'a, T, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S: 'a + <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorageMut.html\" title=\"trait nalgebra::base::storage::RawStorageMut\">RawStorageMut</a>&lt;T, R, C&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"nalgebra/base/iter/struct.MatrixIterMut.html\" title=\"struct nalgebra::base::iter::MatrixIterMut\">MatrixIterMut</a>&lt;'a, T, R, C, S&gt;"]],
"rapier3d":[["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"rapier3d/data/arena/struct.IterMut.html\" title=\"struct rapier3d::data::arena::IterMut\">IterMut</a>&lt;'a, T&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"rapier3d/data/arena/struct.Iter.html\" title=\"struct rapier3d::data::arena::Iter\">Iter</a>&lt;'a, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"rapier3d/data/arena/struct.IntoIter.html\" title=\"struct rapier3d::data::arena::IntoIter\">IntoIter</a>&lt;T&gt;"]],
"regex":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIntoIter.html\" title=\"struct regex::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIter.html\" title=\"struct regex::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIntoIter.html\" title=\"struct regex::bytes::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIter.html\" title=\"struct regex::bytes::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"]],
"regex_automata":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex_automata/struct.PatternSetIter.html\" title=\"struct regex_automata::PatternSetIter\">PatternSetIter</a>&lt;'a&gt;"]],
"slab":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"slab/struct.Iter.html\" title=\"struct slab::Iter\">Iter</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"slab/struct.IterMut.html\" title=\"struct slab::IterMut\">IterMut</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"slab/struct.Drain.html\" title=\"struct slab::Drain\">Drain</a>&lt;'_, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"slab/struct.IntoIter.html\" title=\"struct slab::IntoIter\">IntoIter</a>&lt;T&gt;"]],
"smallvec":[["impl&lt;A: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"smallvec/struct.IntoIter.html\" title=\"struct smallvec::IntoIter\">IntoIter</a>&lt;A&gt;"],["impl&lt;'a, T: 'a + <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"smallvec/struct.Drain.html\" title=\"struct smallvec::Drain\">Drain</a>&lt;'a, T&gt;"]],
"tinyvec":[["impl&lt;'p, A, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"tinyvec/struct.TinyVecSplice.html\" title=\"struct tinyvec::TinyVecSplice\">TinyVecSplice</a>&lt;'p, A, I&gt;<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</span>"],["impl&lt;'a, T: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVecDrain.html\" title=\"struct tinyvec::ArrayVecDrain\">ArrayVecDrain</a>&lt;'a, T&gt;"],["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"enum\" href=\"tinyvec/enum.TinyVecIterator.html\" title=\"enum tinyvec::TinyVecIterator\">TinyVecIterator</a>&lt;A&gt;"],["impl&lt;'p, A, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVecSplice.html\" title=\"struct tinyvec::ArrayVecSplice\">ArrayVecSplice</a>&lt;'p, A, I&gt;<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</span>"],["impl&lt;'p, A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"enum\" href=\"tinyvec/enum.TinyVecDrain.html\" title=\"enum tinyvec::TinyVecDrain\">TinyVecDrain</a>&lt;'p, A&gt;"],["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVecIterator.html\" title=\"struct tinyvec::ArrayVecIterator\">ArrayVecIterator</a>&lt;A&gt;"]],
"vec_map":[["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.ValuesMut.html\" title=\"struct vec_map::ValuesMut\">ValuesMut</a>&lt;'a, V&gt;"],["impl&lt;V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.IntoIter.html\" title=\"struct vec_map::IntoIter\">IntoIter</a>&lt;V&gt;"],["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.Iter.html\" title=\"struct vec_map::Iter\">Iter</a>&lt;'a, V&gt;"],["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.Keys.html\" title=\"struct vec_map::Keys\">Keys</a>&lt;'a, V&gt;"],["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.Drain.html\" title=\"struct vec_map::Drain\">Drain</a>&lt;'a, V&gt;"],["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.IterMut.html\" title=\"struct vec_map::IterMut\">IterMut</a>&lt;'a, V&gt;"],["impl&lt;'a, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"vec_map/struct.Values.html\" title=\"struct vec_map::Values\">Values</a>&lt;'a, V&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()