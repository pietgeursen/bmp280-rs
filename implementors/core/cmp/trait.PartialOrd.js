(function() {var implementors = {};
implementors["byteorder"] = [{"text":"impl PartialOrd&lt;BigEndian&gt; for BigEndian","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LittleEndian&gt; for LittleEndian","synthetic":false,"types":[]}];
implementors["defmt_parser"] = [{"text":"impl PartialOrd&lt;Level&gt; for Level","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T:&nbsp;PartialOrd, N&gt; PartialOrd&lt;GenericArray&lt;T, N&gt;&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["heapless"] = [{"text":"impl&lt;P&gt; PartialOrd&lt;Box&lt;P, Init&gt;&gt; for Box&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Pool,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Data: PartialOrd,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; PartialOrd&lt;Box&lt;T, Init&gt;&gt; for Box&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: PartialOrd,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nb"] = [{"text":"impl&lt;E:&nbsp;PartialOrd&gt; PartialOrd&lt;Error&lt;E&gt;&gt; for Error&lt;E&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl PartialOrd&lt;Ident&gt; for Ident","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl PartialOrd&lt;Lifetime&gt; for Lifetime","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl PartialOrd&lt;B0&gt; for B0","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;B1&gt; for B1","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd + Unsigned + NonZero&gt; PartialOrd&lt;PInt&lt;U&gt;&gt; for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd + Unsigned + NonZero&gt; PartialOrd&lt;NInt&lt;U&gt;&gt; for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Z0&gt; for Z0","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;UTerm&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd, B:&nbsp;PartialOrd&gt; PartialOrd&lt;UInt&lt;U, B&gt;&gt; for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;ATerm&gt; for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V:&nbsp;PartialOrd, A:&nbsp;PartialOrd&gt; PartialOrd&lt;TArr&lt;V, A&gt;&gt; for TArr&lt;V, A&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Greater&gt; for Greater","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Less&gt; for Less","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Equal&gt; for Equal","synthetic":false,"types":[]}];
implementors["void"] = [{"text":"impl&lt;T&gt; PartialOrd&lt;T&gt; for Void","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()