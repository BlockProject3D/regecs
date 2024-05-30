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
use regecs::reflection::component::{Component, PropertyAccessor};
use regecs::reflection::component::list::ComponentList;
use regecs::reflection::component::pool::{ComponentInfo, ComponentRef};
use regecs::reflection::Identifier;
use regecs::reflection::property::value::{Error, Value, ValueString};

#[derive(Component, PropertyAccessor, Default)]
pub struct TestComponent {
    #[property]
    test_field: u32,
    #[property(get=test_field1)]
    test_field1: u32,
    #[property(set=set_test_str, get=test_str)]
    test_str: String
}

impl TestComponent {
    pub fn test_str(&self) -> &str {
        &self.test_str
    }

    pub fn test_field1(&self) -> u32 {
        self.test_field1
    }

    pub fn set_test_str(&mut self, value: String) {
        self.test_field = 42;
        self.test_str = value;
    }
}

impl regecs::component::Component for TestComponent {
    type List = BasicComponentList<Self>;
}

component_pool! {
    #[derive(Default)]
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

impl<V: Value + PropertyAccessorTestComponent> regecs::reflection::component::pool::PropertyAccessor<V> for TestPool {
    fn set_property(&mut self, r: ComponentRef, identifier: Identifier, value: V) -> Result<(), V::ParseError> {
        match r.ty().into_raw() {
            0 => self.tests[r.unchecked_into_ref()].set_property(identifier, value),
            _ => Err(V::ParseError::undefined_property())
        }
    }

    fn get_property(&self, r: ComponentRef, identifier: Identifier, value: V) -> Result<V, V::LoadError> {
        match r.ty().into_raw() {
            0 => self.tests[r.unchecked_into_ref()].get_property(identifier, value),
            _ => Err(V::LoadError::undefined_property())
        }
    }
}

#[test]
fn reflection_main() {
    assert_eq!(TestComponent::PROPERTIES.iter().count(), 3);
    let mut comp = TestComponent::default();
    let test_field = TestComponent::PROPERTIES["test_field"].identifier;
    let test_field1 = TestComponent::PROPERTIES["test_field1"].identifier;
    let test_str = TestComponent::PROPERTIES["test_str"].identifier;
    comp.set_property(test_str, ValueString::from("This is a test")).unwrap();
    let prop = comp.get_property(test_str, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "This is a test");
    let prop = comp.get_property(test_field, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "42");
    let prop = comp.get_property(test_field1, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "0");
    comp.set_property(test_field1, ValueString::from("424242")).unwrap();
    let prop = comp.get_property(test_field1, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "424242");
}

#[test]
fn reflection_pool() {
    use regecs::reflection::component::pool::PropertyAccessor;
    let test_field = TestComponent::PROPERTIES["test_field"].identifier;
    let test_field1 = TestComponent::PROPERTIES["test_field1"].identifier;
    let test_str = TestComponent::PROPERTIES["test_str"].identifier;
    let mut pool = TestPool::default();
    let r = ComponentRef::from_ref::<TestPool, _>(pool.tests.add(TestComponent::default())); //TODO: This also needs reflection support
    assert_eq!(pool.tests.len(), 1);
    pool.set_property(r, test_str, ValueString::from("This is a test")).unwrap();
    let prop = pool.get_property(r, test_str, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "This is a test");
    let prop = pool.get_property(r, test_field, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "42");
    let prop = pool.get_property(r, test_field1, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "0");
    pool.set_property(r, test_field1, ValueString::from("424242")).unwrap();
    let prop = pool.get_property(r, test_field1, ValueString::new()).unwrap();
    assert_eq!(prop.into_inner(), "424242");
    pool.tests.remove(r.into_ref::<TestPool, _>()); //TODO: This also needs reflection support
}
