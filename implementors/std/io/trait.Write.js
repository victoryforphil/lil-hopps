(function() {var implementors = {
"arrayvec":[["impl&lt;const CAP: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.1/std/primitive.u8.html\">u8</a>, CAP&gt;"]],
"bytes":[["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/trait.BufMut.html\" title=\"trait bytes::buf::BufMut\">BufMut</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"bytes/buf/struct.Writer.html\" title=\"struct bytes::buf::Writer\">Writer</a>&lt;B&gt;"]],
"digest":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"digest/core_api/struct.CoreWrapper.html\" title=\"struct digest::core_api::CoreWrapper\">CoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.BufferKindUser.html\" title=\"trait digest::core_api::BufferKindUser\">BufferKindUser</a> + <a class=\"trait\" href=\"digest/core_api/trait.UpdateCore.html\" title=\"trait digest::core_api::UpdateCore\">UpdateCore</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"digest/core_api/struct.RtVariableCoreWrapper.html\" title=\"struct digest::core_api::RtVariableCoreWrapper\">RtVariableCoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.VariableOutputCore.html\" title=\"trait digest::core_api::VariableOutputCore\">VariableOutputCore</a> + <a class=\"trait\" href=\"digest/core_api/trait.UpdateCore.html\" title=\"trait digest::core_api::UpdateCore\">UpdateCore</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</span>"]],
"flate2":[["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibDecoder.html\" title=\"struct flate2::write::ZlibDecoder\">ZlibDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateDecoder.html\" title=\"struct flate2::bufread::DeflateDecoder\">DeflateDecoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.GzEncoder.html\" title=\"struct flate2::read::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.GzDecoder.html\" title=\"struct flate2::write::GzDecoder\">GzDecoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibDecoder.html\" title=\"struct flate2::read::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.GzDecoder.html\" title=\"struct flate2::read::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.MultiGzDecoder.html\" title=\"struct flate2::read::MultiGzDecoder\">MultiGzDecoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.MultiGzDecoder.html\" title=\"struct flate2::write::MultiGzDecoder\">MultiGzDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateEncoder.html\" title=\"struct flate2::write::DeflateEncoder\">DeflateEncoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.DeflateEncoder.html\" title=\"struct flate2::bufread::DeflateEncoder\">DeflateEncoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/struct.CrcWriter.html\" title=\"struct flate2::CrcWriter\">CrcWriter</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.ZlibEncoder.html\" title=\"struct flate2::read::ZlibEncoder\">ZlibEncoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateEncoder.html\" title=\"struct flate2::read::DeflateEncoder\">DeflateEncoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibDecoder.html\" title=\"struct flate2::bufread::ZlibDecoder\">ZlibDecoder</a>&lt;R&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzEncoder.html\" title=\"struct flate2::bufread::GzEncoder\">GzEncoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.ZlibEncoder.html\" title=\"struct flate2::write::ZlibEncoder\">ZlibEncoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.ZlibEncoder.html\" title=\"struct flate2::bufread::ZlibEncoder\">ZlibEncoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.GzEncoder.html\" title=\"struct flate2::write::GzEncoder\">GzEncoder</a>&lt;W&gt;"],["impl&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/bufread/struct.GzDecoder.html\" title=\"struct flate2::bufread::GzDecoder\">GzDecoder</a>&lt;R&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/write/struct.DeflateDecoder.html\" title=\"struct flate2::write::DeflateDecoder\">DeflateDecoder</a>&lt;W&gt;"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"flate2/read/struct.DeflateDecoder.html\" title=\"struct flate2::read::DeflateDecoder\">DeflateDecoder</a>&lt;W&gt;"]],
"futures_util":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"futures_util/io/struct.AllowStdIo.html\" title=\"struct futures_util::io::AllowStdIo\">AllowStdIo</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</span>"]],
"mio":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for &amp;<a class=\"struct\" href=\"mio/unix/pipe/struct.Sender.html\" title=\"struct mio::unix::pipe::Sender\">Sender</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Sender.html\" title=\"struct mio::unix::pipe::Sender\">Sender</a>"]],
"png":[["impl&lt;'a, W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"png/struct.StreamWriter.html\" title=\"struct png::StreamWriter\">StreamWriter</a>&lt;'a, W&gt;"]],
"smithay_client_toolkit":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"smithay_client_toolkit/data_device/struct.WritePipe.html\" title=\"struct smithay_client_toolkit::data_device::WritePipe\">WritePipe</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"smithay_client_toolkit/shm/struct.MemPool.html\" title=\"struct smithay_client_toolkit::shm::MemPool\">MemPool</a>"]],
"socket2":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for &amp;'a <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>"]],
"termcolor":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.StandardStreamLock.html\" title=\"struct termcolor::StandardStreamLock\">StandardStreamLock</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.BufferedStandardStream.html\" title=\"struct termcolor::BufferedStandardStream\">BufferedStandardStream</a>"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.Ansi.html\" title=\"struct termcolor::Ansi\">Ansi</a>&lt;W&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.StandardStream.html\" title=\"struct termcolor::StandardStream\">StandardStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.Buffer.html\" title=\"struct termcolor::Buffer\">Buffer</a>"],["impl&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"termcolor/struct.NoColor.html\" title=\"struct termcolor::NoColor\">NoColor</a>&lt;W&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()