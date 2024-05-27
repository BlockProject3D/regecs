// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use phf::phf_map;
use regecs::component::list::BasicComponentList;
use regecs::component_pool;
use regecs::reflection::component::Component;
use regecs::reflection::component::list::ComponentList;
use regecs::reflection::component::pool::ComponentInfo;
use regecs::reflection::Identifier;
use regecs::reflection::property::list::PropertyList;

pub struct TestComponent {

}

impl regecs::component::Component for TestComponent {
    type List = BasicComponentList<Self>;
}

impl Component for TestComponent {
    const PROPERTIES: &'static PropertyList = &PropertyList::new(phf_map!());
    const NAME: &'static str = "TestComponent";
}

component_pool! {
    pool TestPool {
        tests: TestComponent,
    }
}

impl regecs::reflection::component::pool::ComponentPool for TestPool {
    const COMPONENTS: &'static ComponentList = &ComponentList::new(phf_map! {
        "TestComponent" => ComponentInfo {
            properties: TestComponent::PROPERTIES,
            name: TestComponent::NAME,
            identifier: Identifier::from_raw(0)
        }
    });
}

#[test]
fn reflection_main() {
    use regecs::reflection::component::pool::ComponentPool;
    TestPool::get_ref(regecs::component::ComponentRef::<TestComponent>::new(0));
}
