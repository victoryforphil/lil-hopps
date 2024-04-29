(function() {var implementors = {
"allocator_api2":[["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;T&gt; for <a class=\"struct\" href=\"allocator_api2/boxed/struct.Box.html\" title=\"struct allocator_api2::boxed::Box\">Box</a>&lt;T, A&gt;"]],
"arrayvec":[["impl&lt;const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayString.html\" title=\"struct arrayvec::ArrayString\">ArrayString</a>&lt;CAP&gt;"],["impl&lt;T, const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.slice.html\">[T]</a>&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;T, CAP&gt;"]],
"crossbeam_epoch":[["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"crossbeam_epoch/trait.Pointable.html\" title=\"trait crossbeam_epoch::Pointable\">Pointable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;"]],
"nalgebra":[["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 5]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">4</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 6]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">4</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 2]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">6</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 3]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 3]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">5</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 2]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 5]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 4]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 3]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">6</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 5]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">5</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 4]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">5</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 2]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">4</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 6]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 6]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">6</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 2]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 5]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 6]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">5</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 4]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">6</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 2]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">5</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 3]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U2.html\" title=\"type nalgebra::base::dimension::U2\">U2</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 4]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">4</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 6]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 4]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 5]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">6</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U5.html\" title=\"type nalgebra::base::dimension::U5\">U5</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U6.html\" title=\"type nalgebra::base::dimension::U6\">U6</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">[T; 3]</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.array.html\">4</a>]&gt; for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.U3.html\" title=\"type nalgebra::base::dimension::U3\">U3</a>, <a class=\"type\" href=\"nalgebra/base/dimension/type.U4.html\" title=\"type nalgebra::base::dimension::U4\">U4</a>&gt; + <a class=\"trait\" href=\"nalgebra/base/storage/trait.IsContiguous.html\" title=\"trait nalgebra::base::storage::IsContiguous\">IsContiguous</a>,</div>"]],
"smallvec":[["impl&lt;A: <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;[&lt;A as <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt;::<a class=\"associatedtype\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a>]&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()