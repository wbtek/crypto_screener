
// The MIT License (MIT)
//
// Copyright (c) 2024 Greg Slocum, WBTek,
// a division of WhiteBear Family, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use yew::{html, Html, Context};
use super::component::Model;
use super::message::Msg;

pub fn about_view(ctx: &Context<Model>) -> Html {
    html! {
        <div class="modal">
            <div class="modal-content">
                <span class="close" onclick={ctx.link().callback(|_| Msg::ToggleAbout)}>{ "\u{00D7}" }</span> // multiply char
                <h2> { "About WBTek Crypto Screener" } </h2>
                <p>
                    { "Click on header buttons to sort data, and click on individual cells to highlight them." } <br />
                    <br />
                    { "Built with Rust and Yew, compiled to WebAssembly (WASM)." } <br />
                    <br />
                    { "Source and documentation: " }
                        <a href="https://wbtek.github.io"
                            target="_blank">{ "https://wbtek.github.io" }
                    </a>
                    <br />
                    { "WBTek's Homepage: " }
                        <a href="https://wbtek.slocum.net"
                            target="_blank">{ "https://wbtek.slocum.net" }
                    </a>

                    <hr />
                    <br />
                    { "The MIT License (MIT)" } <br />
                    <br />
                    { "Copyright (c) 2024 Greg Slocum, WBTek," } <br />
                    { "a division of WhiteBear Family, Inc." } <br />
                    <br />
                    { "Permission is hereby granted, free of charge, to any person obtaining a copy
                    of this software and associated documentation files (the \"Software\"), to deal
                    in the Software without restriction, including without limitation the rights
                    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                    copies of the Software, and to permit persons to whom the Software is
                    furnished to do so, subject to the following conditions:" } <br />
                    <br />
                    { "The above copyright notice and this permission notice shall be included in all
                    copies or substantial portions of the Software." } <br />
                    <br />
                    { "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                    SOFTWARE." } <br />
                    <br />
                    <hr />
                    <br />
                </p>
                <p style="font-size: 0.7em;">{ "(0077)" }</p>
            </div>
        </div>
    }
}
