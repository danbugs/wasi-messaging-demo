#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(clippy::all)]
pub mod tracing {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    /// An error resource type.
    /// Currently, this provides only one function to return a string representation
    /// of the error. In the future, this will be extended to provide more information.
    pub type Error = u32;
    pub trait Tracing: Sized {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn drop_error<'life0, 'async_trait>(
            &'life0 mut self,
            e: Error,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn trace<'life0, 'async_trait>(
            &'life0 mut self,
            e: Error,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<String>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: Tracing + Send,
    {
        let mut inst = linker.instance("tracing")?;
        inst.func_wrap_async(
            "drop-error",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (Error,)| Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.drop_error(arg0).await;
                r
            }),
        )?;
        inst.func_wrap_async(
            "trace",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (Error,)| Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.trace(arg0).await;
                Ok((r?,))
            }),
        )?;
        Ok(())
    }
}
#[allow(clippy::all)]
pub mod messaging_types {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub type Error = super::tracing::Error;
    /// A subscription token that allows receives from a specific subscription
    pub type SubscriptionToken = String;
    /// An event type that follows the CloudEvents specification (https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md). We
    /// assume the type of the data is a byte sequence. It is up to the data schema to determine what type of the data payload the event
    /// contains.
    #[component(record)]
    pub struct EventParam<'a> {
        #[component(name = "specversion")]
        pub specversion: &'a str,
        #[component(name = "ty")]
        pub ty: &'a str,
        #[component(name = "source")]
        pub source: &'a str,
        #[component(name = "id")]
        pub id: &'a str,
        #[component(name = "data")]
        pub data: Option<&'a [u8]>,
        #[component(name = "datacontenttype")]
        pub datacontenttype: Option<&'a str>,
        #[component(name = "dataschema")]
        pub dataschema: Option<&'a str>,
        #[component(name = "subject")]
        pub subject: Option<&'a str>,
        #[component(name = "time")]
        pub time: Option<&'a str>,
        #[component(name = "extensions")]
        pub extensions: Option<&'a [(&'a str, &'a str)]>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for EventParam<'a> {
        #[inline]
        fn clone(&self) -> EventParam<'a> {
            EventParam {
                specversion: ::core::clone::Clone::clone(&self.specversion),
                ty: ::core::clone::Clone::clone(&self.ty),
                source: ::core::clone::Clone::clone(&self.source),
                id: ::core::clone::Clone::clone(&self.id),
                data: ::core::clone::Clone::clone(&self.data),
                datacontenttype: ::core::clone::Clone::clone(&self.datacontenttype),
                dataschema: ::core::clone::Clone::clone(&self.dataschema),
                subject: ::core::clone::Clone::clone(&self.subject),
                time: ::core::clone::Clone::clone(&self.time),
                extensions: ::core::clone::Clone::clone(&self.extensions),
            }
        }
    }
    unsafe impl<'a> wasmtime::component::Lower for EventParam<'a> {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            wasmtime::component::Lower::lower(
                &self.specversion,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).specversion)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.ty,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).ty)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.source,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).source)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.id,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).id)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.data,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).data)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.datacontenttype,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).datacontenttype)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.dataschema,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).dataschema)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.subject,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).subject)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.time,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).time)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.extensions,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).extensions)
                        }
                    }
                },
            )?;
            Ok(())
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset
                    % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize)
                    == 0)
                {
                    ::core::panicking::panic(
                        "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                    )
                }
            }
            wasmtime::component::Lower::store(
                &self.specversion,
                memory,
                <&'a str as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.ty,
                memory,
                <&'a str as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.source,
                memory,
                <&'a str as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.id,
                memory,
                <&'a str as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.data,
                memory,
                <Option<&'a [u8]> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.datacontenttype,
                memory,
                <Option<&'a str> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.dataschema,
                memory,
                <Option<&'a str> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.subject,
                memory,
                <Option<&'a str> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.time,
                memory,
                <Option<&'a str> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.extensions,
                memory,
                <Option<
                    &'a [(&'a str, &'a str)],
                > as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            Ok(())
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerEventParam<
            T0: Copy,
            T1: Copy,
            T2: Copy,
            T3: Copy,
            T4: Copy,
            T5: Copy,
            T6: Copy,
            T7: Copy,
            T8: Copy,
            T9: Copy,
        > {
            specversion: T0,
            ty: T1,
            source: T2,
            id: T3,
            data: T4,
            datacontenttype: T5,
            dataschema: T6,
            subject: T7,
            time: T8,
            extensions: T9,
            _align: [wasmtime::ValRaw; 0],
        }
        #[automatically_derived]
        impl<
            T0: ::core::clone::Clone + Copy,
            T1: ::core::clone::Clone + Copy,
            T2: ::core::clone::Clone + Copy,
            T3: ::core::clone::Clone + Copy,
            T4: ::core::clone::Clone + Copy,
            T5: ::core::clone::Clone + Copy,
            T6: ::core::clone::Clone + Copy,
            T7: ::core::clone::Clone + Copy,
            T8: ::core::clone::Clone + Copy,
            T9: ::core::clone::Clone + Copy,
        > ::core::clone::Clone
        for LowerEventParam<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
            #[inline]
            fn clone(&self) -> LowerEventParam<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
                LowerEventParam {
                    specversion: ::core::clone::Clone::clone(&self.specversion),
                    ty: ::core::clone::Clone::clone(&self.ty),
                    source: ::core::clone::Clone::clone(&self.source),
                    id: ::core::clone::Clone::clone(&self.id),
                    data: ::core::clone::Clone::clone(&self.data),
                    datacontenttype: ::core::clone::Clone::clone(&self.datacontenttype),
                    dataschema: ::core::clone::Clone::clone(&self.dataschema),
                    subject: ::core::clone::Clone::clone(&self.subject),
                    time: ::core::clone::Clone::clone(&self.time),
                    extensions: ::core::clone::Clone::clone(&self.extensions),
                    _align: ::core::clone::Clone::clone(&self._align),
                }
            }
        }
        #[automatically_derived]
        impl<
            T0: ::core::marker::Copy + Copy,
            T1: ::core::marker::Copy + Copy,
            T2: ::core::marker::Copy + Copy,
            T3: ::core::marker::Copy + Copy,
            T4: ::core::marker::Copy + Copy,
            T5: ::core::marker::Copy + Copy,
            T6: ::core::marker::Copy + Copy,
            T7: ::core::marker::Copy + Copy,
            T8: ::core::marker::Copy + Copy,
            T9: ::core::marker::Copy + Copy,
        > ::core::marker::Copy
        for LowerEventParam<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {}
        unsafe impl<'a> wasmtime::component::ComponentType for EventParam<'a> {
            type Lower = LowerEventParam<
                <&'a str as wasmtime::component::ComponentType>::Lower,
                <&'a str as wasmtime::component::ComponentType>::Lower,
                <&'a str as wasmtime::component::ComponentType>::Lower,
                <&'a str as wasmtime::component::ComponentType>::Lower,
                <Option<&'a [u8]> as wasmtime::component::ComponentType>::Lower,
                <Option<&'a str> as wasmtime::component::ComponentType>::Lower,
                <Option<&'a str> as wasmtime::component::ComponentType>::Lower,
                <Option<&'a str> as wasmtime::component::ComponentType>::Lower,
                <Option<&'a str> as wasmtime::component::ComponentType>::Lower,
                <Option<
                    &'a [(&'a str, &'a str)],
                > as wasmtime::component::ComponentType>::Lower,
            >;
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                &[
                    <&'a str as wasmtime::component::ComponentType>::ABI,
                    <&'a str as wasmtime::component::ComponentType>::ABI,
                    <&'a str as wasmtime::component::ComponentType>::ABI,
                    <&'a str as wasmtime::component::ComponentType>::ABI,
                    <Option<&'a [u8]> as wasmtime::component::ComponentType>::ABI,
                    <Option<&'a str> as wasmtime::component::ComponentType>::ABI,
                    <Option<&'a str> as wasmtime::component::ComponentType>::ABI,
                    <Option<&'a str> as wasmtime::component::ComponentType>::ABI,
                    <Option<&'a str> as wasmtime::component::ComponentType>::ABI,
                    <Option<
                        &'a [(&'a str, &'a str)],
                    > as wasmtime::component::ComponentType>::ABI,
                ],
            );
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime::component::__internal::typecheck_record(
                    ty,
                    types,
                    &[
                        (
                            "specversion",
                            <&'a str as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "ty",
                            <&'a str as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "source",
                            <&'a str as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "id",
                            <&'a str as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "data",
                            <Option<
                                &'a [u8],
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "datacontenttype",
                            <Option<
                                &'a str,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "dataschema",
                            <Option<
                                &'a str,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "subject",
                            <Option<
                                &'a str,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "time",
                            <Option<
                                &'a str,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "extensions",
                            <Option<
                                &'a [(&'a str, &'a str)],
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                    ],
                )
            }
        }
    };
    impl<'a> core::fmt::Debug for EventParam<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("EventParam")
                .field("specversion", &self.specversion)
                .field("ty", &self.ty)
                .field("source", &self.source)
                .field("id", &self.id)
                .field("data", &self.data)
                .field("datacontenttype", &self.datacontenttype)
                .field("dataschema", &self.dataschema)
                .field("subject", &self.subject)
                .field("time", &self.time)
                .field("extensions", &self.extensions)
                .finish()
        }
    }
    /// An event type that follows the CloudEvents specification (https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md). We
    /// assume the type of the data is a byte sequence. It is up to the data schema to determine what type of the data payload the event
    /// contains.
    #[component(record)]
    pub struct EventResult {
        #[component(name = "specversion")]
        pub specversion: String,
        #[component(name = "ty")]
        pub ty: String,
        #[component(name = "source")]
        pub source: String,
        #[component(name = "id")]
        pub id: String,
        #[component(name = "data")]
        pub data: Option<Vec<u8>>,
        #[component(name = "datacontenttype")]
        pub datacontenttype: Option<String>,
        #[component(name = "dataschema")]
        pub dataschema: Option<String>,
        #[component(name = "subject")]
        pub subject: Option<String>,
        #[component(name = "time")]
        pub time: Option<String>,
        #[component(name = "extensions")]
        pub extensions: Option<Vec<(String, String)>>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventResult {
        #[inline]
        fn clone(&self) -> EventResult {
            EventResult {
                specversion: ::core::clone::Clone::clone(&self.specversion),
                ty: ::core::clone::Clone::clone(&self.ty),
                source: ::core::clone::Clone::clone(&self.source),
                id: ::core::clone::Clone::clone(&self.id),
                data: ::core::clone::Clone::clone(&self.data),
                datacontenttype: ::core::clone::Clone::clone(&self.datacontenttype),
                dataschema: ::core::clone::Clone::clone(&self.dataschema),
                subject: ::core::clone::Clone::clone(&self.subject),
                time: ::core::clone::Clone::clone(&self.time),
                extensions: ::core::clone::Clone::clone(&self.extensions),
            }
        }
    }
    unsafe impl wasmtime::component::Lower for EventResult {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            wasmtime::component::Lower::lower(
                &self.specversion,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).specversion)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.ty,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).ty)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.source,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).source)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.id,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).id)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.data,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).data)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.datacontenttype,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).datacontenttype)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.dataschema,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).dataschema)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.subject,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).subject)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.time,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).time)
                        }
                    }
                },
            )?;
            wasmtime::component::Lower::lower(
                &self.extensions,
                store,
                options,
                {
                    #[allow(unused_unsafe)]
                    {
                        unsafe {
                            use ::wasmtime::component::__internal::MaybeUninitExt;
                            let m: &mut std::mem::MaybeUninit<_> = dst;
                            m.map(|p| &raw mut (*p).extensions)
                        }
                    }
                },
            )?;
            Ok(())
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset
                    % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize)
                    == 0)
                {
                    ::core::panicking::panic(
                        "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                    )
                }
            }
            wasmtime::component::Lower::store(
                &self.specversion,
                memory,
                <String as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.ty,
                memory,
                <String as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.source,
                memory,
                <String as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.id,
                memory,
                <String as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.data,
                memory,
                <Option<Vec<u8>> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.datacontenttype,
                memory,
                <Option<String> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.dataschema,
                memory,
                <Option<String> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.subject,
                memory,
                <Option<String> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.time,
                memory,
                <Option<String> as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            wasmtime::component::Lower::store(
                &self.extensions,
                memory,
                <Option<
                    Vec<(String, String)>,
                > as wasmtime::component::ComponentType>::ABI
                    .next_field32_size(&mut offset),
            )?;
            Ok(())
        }
    }
    unsafe impl wasmtime::component::Lift for EventResult {
        #[inline]
        fn lift(
            store: &wasmtime::component::__internal::StoreOpaque,
            options: &wasmtime::component::__internal::Options,
            src: &Self::Lower,
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            Ok(Self {
                specversion: <String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.specversion,
                )?,
                ty: <String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.ty,
                )?,
                source: <String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.source,
                )?,
                id: <String as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.id,
                )?,
                data: <Option<
                    Vec<u8>,
                > as wasmtime::component::Lift>::lift(store, options, &src.data)?,
                datacontenttype: <Option<
                    String,
                > as wasmtime::component::Lift>::lift(
                    store,
                    options,
                    &src.datacontenttype,
                )?,
                dataschema: <Option<
                    String,
                > as wasmtime::component::Lift>::lift(store, options, &src.dataschema)?,
                subject: <Option<
                    String,
                > as wasmtime::component::Lift>::lift(store, options, &src.subject)?,
                time: <Option<
                    String,
                > as wasmtime::component::Lift>::lift(store, options, &src.time)?,
                extensions: <Option<
                    Vec<(String, String)>,
                > as wasmtime::component::Lift>::lift(store, options, &src.extensions)?,
            })
        }
        #[inline]
        fn load(
            memory: &wasmtime::component::__internal::Memory,
            bytes: &[u8],
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            if true {
                if !((bytes.as_ptr() as usize)
                    % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize)
                    == 0)
                {
                    ::core::panicking::panic(
                        "assertion failed: (bytes.as_ptr() as usize) %\\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                    )
                }
            }
            let mut offset = 0;
            Ok(Self {
                specversion: <String as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<String as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                )?,
                ty: <String as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<String as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                )?,
                source: <String as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<String as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                )?,
                id: <String as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<String as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                )?,
                data: <Option<
                    Vec<u8>,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<Vec<u8>> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        Vec<u8>,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
                datacontenttype: <Option<
                    String,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<String> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        String,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
                dataschema: <Option<
                    String,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<String> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        String,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
                subject: <Option<
                    String,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<String> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        String,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
                time: <Option<
                    String,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<String> as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        String,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
                extensions: <Option<
                    Vec<(String, String)>,
                > as wasmtime::component::Lift>::load(
                    memory,
                    &bytes[<Option<
                        Vec<(String, String)>,
                    > as wasmtime::component::ComponentType>::ABI
                        .next_field32_size(
                            &mut offset,
                        )..][..<Option<
                        Vec<(String, String)>,
                    > as wasmtime::component::ComponentType>::SIZE32],
                )?,
            })
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerEventResult<
            T0: Copy,
            T1: Copy,
            T2: Copy,
            T3: Copy,
            T4: Copy,
            T5: Copy,
            T6: Copy,
            T7: Copy,
            T8: Copy,
            T9: Copy,
        > {
            specversion: T0,
            ty: T1,
            source: T2,
            id: T3,
            data: T4,
            datacontenttype: T5,
            dataschema: T6,
            subject: T7,
            time: T8,
            extensions: T9,
            _align: [wasmtime::ValRaw; 0],
        }
        #[automatically_derived]
        impl<
            T0: ::core::clone::Clone + Copy,
            T1: ::core::clone::Clone + Copy,
            T2: ::core::clone::Clone + Copy,
            T3: ::core::clone::Clone + Copy,
            T4: ::core::clone::Clone + Copy,
            T5: ::core::clone::Clone + Copy,
            T6: ::core::clone::Clone + Copy,
            T7: ::core::clone::Clone + Copy,
            T8: ::core::clone::Clone + Copy,
            T9: ::core::clone::Clone + Copy,
        > ::core::clone::Clone
        for LowerEventResult<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
            #[inline]
            fn clone(&self) -> LowerEventResult<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
                LowerEventResult {
                    specversion: ::core::clone::Clone::clone(&self.specversion),
                    ty: ::core::clone::Clone::clone(&self.ty),
                    source: ::core::clone::Clone::clone(&self.source),
                    id: ::core::clone::Clone::clone(&self.id),
                    data: ::core::clone::Clone::clone(&self.data),
                    datacontenttype: ::core::clone::Clone::clone(&self.datacontenttype),
                    dataschema: ::core::clone::Clone::clone(&self.dataschema),
                    subject: ::core::clone::Clone::clone(&self.subject),
                    time: ::core::clone::Clone::clone(&self.time),
                    extensions: ::core::clone::Clone::clone(&self.extensions),
                    _align: ::core::clone::Clone::clone(&self._align),
                }
            }
        }
        #[automatically_derived]
        impl<
            T0: ::core::marker::Copy + Copy,
            T1: ::core::marker::Copy + Copy,
            T2: ::core::marker::Copy + Copy,
            T3: ::core::marker::Copy + Copy,
            T4: ::core::marker::Copy + Copy,
            T5: ::core::marker::Copy + Copy,
            T6: ::core::marker::Copy + Copy,
            T7: ::core::marker::Copy + Copy,
            T8: ::core::marker::Copy + Copy,
            T9: ::core::marker::Copy + Copy,
        > ::core::marker::Copy
        for LowerEventResult<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {}
        unsafe impl wasmtime::component::ComponentType for EventResult {
            type Lower = LowerEventResult<
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
                <Option<Vec<u8>> as wasmtime::component::ComponentType>::Lower,
                <Option<String> as wasmtime::component::ComponentType>::Lower,
                <Option<String> as wasmtime::component::ComponentType>::Lower,
                <Option<String> as wasmtime::component::ComponentType>::Lower,
                <Option<String> as wasmtime::component::ComponentType>::Lower,
                <Option<
                    Vec<(String, String)>,
                > as wasmtime::component::ComponentType>::Lower,
            >;
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                &[
                    <String as wasmtime::component::ComponentType>::ABI,
                    <String as wasmtime::component::ComponentType>::ABI,
                    <String as wasmtime::component::ComponentType>::ABI,
                    <String as wasmtime::component::ComponentType>::ABI,
                    <Option<Vec<u8>> as wasmtime::component::ComponentType>::ABI,
                    <Option<String> as wasmtime::component::ComponentType>::ABI,
                    <Option<String> as wasmtime::component::ComponentType>::ABI,
                    <Option<String> as wasmtime::component::ComponentType>::ABI,
                    <Option<String> as wasmtime::component::ComponentType>::ABI,
                    <Option<
                        Vec<(String, String)>,
                    > as wasmtime::component::ComponentType>::ABI,
                ],
            );
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime::component::__internal::typecheck_record(
                    ty,
                    types,
                    &[
                        (
                            "specversion",
                            <String as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "ty",
                            <String as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "source",
                            <String as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "id",
                            <String as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "data",
                            <Option<
                                Vec<u8>,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "datacontenttype",
                            <Option<
                                String,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "dataschema",
                            <Option<
                                String,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "subject",
                            <Option<
                                String,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "time",
                            <Option<
                                String,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                        (
                            "extensions",
                            <Option<
                                Vec<(String, String)>,
                            > as wasmtime::component::ComponentType>::typecheck,
                        ),
                    ],
                )
            }
        }
    };
    impl core::fmt::Debug for EventResult {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("EventResult")
                .field("specversion", &self.specversion)
                .field("ty", &self.ty)
                .field("source", &self.source)
                .field("id", &self.id)
                .field("data", &self.data)
                .field("datacontenttype", &self.datacontenttype)
                .field("dataschema", &self.dataschema)
                .field("subject", &self.subject)
                .field("time", &self.time)
                .field("extensions", &self.extensions)
                .finish()
        }
    }
    /// Channels specify where a published message should land. There are two types of channels:
    /// - queue: competitive consumers, and
    /// - topic: non-competitive consumers.
    #[component(variant)]
    pub enum Channel {
        #[component(name = "queue")]
        Queue(String),
        #[component(name = "topic")]
        Topic(String),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Channel {
        #[inline]
        fn clone(&self) -> Channel {
            match self {
                Channel::Queue(__self_0) => {
                    Channel::Queue(::core::clone::Clone::clone(__self_0))
                }
                Channel::Topic(__self_0) => {
                    Channel::Topic(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    unsafe impl wasmtime::component::Lower for Channel {
        #[inline]
        fn lower<T>(
            &self,
            store: &mut wasmtime::StoreContextMut<T>,
            options: &wasmtime::component::__internal::Options,
            dst: &mut std::mem::MaybeUninit<Self::Lower>,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            match self {
                Self::Queue(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                        .write(wasmtime::ValRaw::u32(0u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Queue)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
                Self::Topic(value) => {
                    {
                        #[allow(unused_unsafe)]
                        {
                            unsafe {
                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                m.map(|p| &raw mut (*p).tag)
                            }
                        }
                    }
                        .write(wasmtime::ValRaw::u32(1u32));
                    unsafe {
                        wasmtime::component::__internal::lower_payload(
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).payload)
                                    }
                                }
                            },
                            |payload| {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = payload;
                                        m.map(|p| &raw mut (*p).Topic)
                                    }
                                }
                            },
                            |dst| value.lower(store, options, dst),
                        )
                    }
                }
            }
        }
        #[inline]
        fn store<T>(
            &self,
            memory: &mut wasmtime::component::__internal::MemoryMut<'_, T>,
            mut offset: usize,
        ) -> wasmtime::component::__internal::anyhow::Result<()> {
            if true {
                if !(offset
                    % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize)
                    == 0)
                {
                    ::core::panicking::panic(
                        "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                    )
                }
            }
            match self {
                Self::Queue(value) => {
                    *memory.get::<1usize>(offset) = 0u8.to_le_bytes();
                    value
                        .store(
                            memory,
                            offset
                                + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                        )
                }
                Self::Topic(value) => {
                    *memory.get::<1usize>(offset) = 1u8.to_le_bytes();
                    value
                        .store(
                            memory,
                            offset
                                + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                        )
                }
            }
        }
    }
    unsafe impl wasmtime::component::Lift for Channel {
        #[inline]
        fn lift(
            store: &wasmtime::component::__internal::StoreOpaque,
            options: &wasmtime::component::__internal::Options,
            src: &Self::Lower,
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            Ok(
                match src.tag.get_u32() {
                    0u32 => {
                        Self::Queue(
                            <String as wasmtime::component::Lift>::lift(
                                store,
                                options,
                                unsafe { &src.payload.Queue },
                            )?,
                        )
                    }
                    1u32 => {
                        Self::Topic(
                            <String as wasmtime::component::Lift>::lift(
                                store,
                                options,
                                unsafe { &src.payload.Topic },
                            )?,
                        )
                    }
                    discrim => {
                        return ::anyhow::__private::Err(
                            ::anyhow::Error::msg({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["unexpected discriminant: "],
                                        &[::core::fmt::ArgumentV1::new_display(&discrim)],
                                    ),
                                );
                                res
                            }),
                        );
                    }
                },
            )
        }
        #[inline]
        fn load(
            memory: &wasmtime::component::__internal::Memory,
            bytes: &[u8],
        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
            let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
            if true {
                if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                    ::core::panicking::panic(
                        "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                    )
                }
            }
            let discrim = bytes[0];
            let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
            let payload = &bytes[payload_offset..];
            Ok(
                match discrim {
                    0u8 => {
                        Self::Queue(
                            <String as wasmtime::component::Lift>::load(
                                memory,
                                &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        )
                    }
                    1u8 => {
                        Self::Topic(
                            <String as wasmtime::component::Lift>::load(
                                memory,
                                &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        )
                    }
                    discrim => {
                        return ::anyhow::__private::Err(
                            ::anyhow::Error::msg({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["unexpected discriminant: "],
                                        &[::core::fmt::ArgumentV1::new_display(&discrim)],
                                    ),
                                );
                                res
                            }),
                        );
                    }
                },
            )
        }
    }
    const _: () = {
        #[doc(hidden)]
        #[repr(C)]
        pub struct LowerChannel<T0: Copy, T1: Copy> {
            tag: wasmtime::ValRaw,
            payload: LowerPayloadChannel<T0, T1>,
        }
        #[automatically_derived]
        impl<
            T0: ::core::clone::Clone + Copy,
            T1: ::core::clone::Clone + Copy,
        > ::core::clone::Clone for LowerChannel<T0, T1> {
            #[inline]
            fn clone(&self) -> LowerChannel<T0, T1> {
                LowerChannel {
                    tag: ::core::clone::Clone::clone(&self.tag),
                    payload: ::core::clone::Clone::clone(&self.payload),
                }
            }
        }
        #[automatically_derived]
        impl<
            T0: ::core::marker::Copy + Copy,
            T1: ::core::marker::Copy + Copy,
        > ::core::marker::Copy for LowerChannel<T0, T1> {}
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[repr(C)]
        union LowerPayloadChannel<T0: Copy, T1: Copy> {
            Queue: T0,
            Topic: T1,
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<
            T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
            T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
        > ::core::clone::Clone for LowerPayloadChannel<T0, T1> {
            #[inline]
            fn clone(&self) -> LowerPayloadChannel<T0, T1> {
                let _: ::core::clone::AssertParamIsCopy<Self>;
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<
            T0: ::core::marker::Copy + Copy,
            T1: ::core::marker::Copy + Copy,
        > ::core::marker::Copy for LowerPayloadChannel<T0, T1> {}
        unsafe impl wasmtime::component::ComponentType for Channel {
            type Lower = LowerChannel<
                <String as wasmtime::component::ComponentType>::Lower,
                <String as wasmtime::component::ComponentType>::Lower,
            >;
            #[inline]
            fn typecheck(
                ty: &wasmtime::component::__internal::InterfaceType,
                types: &wasmtime::component::__internal::ComponentTypes,
            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                wasmtime::component::__internal::typecheck_variant(
                    ty,
                    types,
                    &[
                        (
                            "queue",
                            Some(
                                <String as wasmtime::component::ComponentType>::typecheck,
                            ),
                        ),
                        (
                            "topic",
                            Some(
                                <String as wasmtime::component::ComponentType>::typecheck,
                            ),
                        ),
                    ],
                )
            }
            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                &[
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                    Some(<String as wasmtime::component::ComponentType>::ABI),
                ],
            );
        }
        unsafe impl wasmtime::component::__internal::ComponentVariant for Channel {
            const CASES: &'static [Option<
                wasmtime::component::__internal::CanonicalAbiInfo,
            >] = &[
                Some(<String as wasmtime::component::ComponentType>::ABI),
                Some(<String as wasmtime::component::ComponentType>::ABI),
            ];
        }
    };
    impl core::fmt::Debug for Channel {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Channel::Queue(e) => f.debug_tuple("Channel::Queue").field(e).finish(),
                Channel::Topic(e) => f.debug_tuple("Channel::Topic").field(e).finish(),
            }
        }
    }
    /// A broker type that allows the exchange of messages.
    pub type Broker = u32;
    pub trait MessagingTypes: Sized {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn drop_broker<'life0, 'async_trait>(
            &'life0 mut self,
            b: Broker,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn open_broker<'life0, 'async_trait>(
            &'life0 mut self,
            name: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<Result<Broker, Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: MessagingTypes + Send,
    {
        let mut inst = linker.instance("messaging-types")?;
        inst.func_wrap_async(
            "drop-broker",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (Broker,)| Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.drop_broker(arg0).await;
                r
            }),
        )?;
        inst.func_wrap_async(
            "open-broker",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (String,)| Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.open_broker(arg0).await;
                Ok((r?,))
            }),
        )?;
        Ok(())
    }
}
#[allow(clippy::all)]
pub mod producer {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub type Broker = super::messaging_types::Broker;
    pub type Channel = super::messaging_types::Channel;
    pub type Event = super::messaging_types::EventResult;
    pub type Error = super::tracing::Error;
    pub trait Producer: Sized {
        /// Publishes an event to a channel in a broker.
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn publish<'life0, 'async_trait>(
            &'life0 mut self,
            b: Broker,
            c: Channel,
            e: Event,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<Result<(), Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: Producer + Send,
    {
        let mut inst = linker.instance("producer")?;
        inst.func_wrap_async(
            "publish",
            move |
                mut caller: wasmtime::StoreContextMut<'_, T>,
                (arg0, arg1, arg2): (Broker, Channel, Event)|
            Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.publish(arg0, arg1, arg2).await;
                Ok((r?,))
            }),
        )?;
        Ok(())
    }
}
#[allow(clippy::all)]
pub mod consumer {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub type Broker = super::messaging_types::Broker;
    pub type Channel = super::messaging_types::Channel;
    pub type SubscriptionToken = super::messaging_types::SubscriptionToken;
    pub type Error = super::tracing::Error;
    pub trait Consumer: Sized {
        /// Subscribes to a channel in a broker.
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn subscribe<'life0, 'async_trait>(
            &'life0 mut self,
            b: Broker,
            c: Channel,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<Result<SubscriptionToken, Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
        /// Unsubscribes from a channel via subscription token.
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn unsubscribe<'life0, 'async_trait>(
            &'life0 mut self,
            b: Broker,
            st: SubscriptionToken,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<Result<(), Error>>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: Consumer + Send,
    {
        let mut inst = linker.instance("consumer")?;
        inst.func_wrap_async(
            "subscribe",
            move |
                mut caller: wasmtime::StoreContextMut<'_, T>,
                (arg0, arg1): (Broker, Channel)|
            Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.subscribe(arg0, arg1).await;
                Ok((r?,))
            }),
        )?;
        inst.func_wrap_async(
            "unsubscribe",
            move |
                mut caller: wasmtime::StoreContextMut<'_, T>,
                (arg0, arg1): (Broker, SubscriptionToken)|
            Box::new(async move {
                let host = get(caller.data_mut());
                let r = host.unsubscribe(arg0, arg1).await;
                Ok((r?,))
            }),
        )?;
        Ok(())
    }
}
#[allow(clippy::all)]
pub mod handler {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    pub type EventParam<'a> = super::messaging_types::EventParam<'a>;
    pub type EventResult = super::messaging_types::EventResult;
    pub type Error = super::tracing::Error;
    pub struct Handler {
        on_receive: wasmtime::component::Func,
    }
    impl Handler {
        pub fn new(
            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
        ) -> anyhow::Result<Handler> {
            let on_receive = *__exports
                .typed_func::<(EventParam<'_>,), (Result<(), Error>,)>("on-receive")?
                .func();
            Ok(Handler { on_receive })
        }
        /// Creates an on-receive handler push-based message delivery.
        pub async fn call_on_receive<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
            arg0: EventParam<'_>,
        ) -> anyhow::Result<Result<(), Error>>
        where
            <S as wasmtime::AsContext>::Data: Send,
        {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<
                    (EventParam<'_>,),
                    (Result<(), Error>,),
                >::new_unchecked(self.on_receive)
            };
            let (ret0,) = callee.call_async(store.as_context_mut(), (arg0,)).await?;
            callee.post_return_async(store.as_context_mut()).await?;
            Ok(ret0)
        }
    }
}
pub struct Messaging {
    handler: handler::Handler,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl Messaging {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()>
        where
            U: tracing::Tracing + messaging_types::MessagingTypes + producer::Producer
                + consumer::Consumer + Send,
            T: Send,
        {
            tracing::add_to_linker(linker, get)?;
            messaging_types::add_to_linker(linker, get)?;
            producer::add_to_linker(linker, get)?;
            consumer::add_to_linker(linker, get)?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub async fn instantiate_async<T: Send>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> anyhow::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate_async(&mut store, component).await?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub async fn instantiate_pre<T: Send>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> anyhow::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate_async(&mut store).await?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> anyhow::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let handler = handler::Handler::new(
                &mut __exports
                    .instance("handler")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            ::core::fmt::Arguments::new_v1(
                                &["exported instance `handler` not present"],
                                &[],
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(Messaging { handler })
        }
        pub fn handler(&self) -> &handler::Handler {
            &self.handler
        }
    }
};
const _: &str = "default interface tracing {\n\t/// An error resource type.\n\t/// Currently, this provides only one function to return a string representation\n\t/// of the error. In the future, this will be extended to provide more information.\n\t// TODO: switch to `resource error { ... }`\n\ttype error = u32\n\tdrop-error: func(e: error)\n\ttrace: func(e: error) -> string\n}";
const _: &str = "default world messaging {\n\timport producer: pkg.producer\n\timport consumer: pkg.consumer\n\texport handler: pkg.handler\n}";
const _: &str = "/// An interface for a generic consumer.\ndefault interface consumer {\n    use pkg.types.{broker, channel, subscription-token}\n    use err.tracing.{error}\n\n\t/// Subscribes to a channel in a broker.\n  \tsubscribe: func(b: broker, c: channel) -> result<subscription-token, error>\n\n\t/// Unsubscribes from a channel via subscription token.\n  \tunsubscribe: func(b: broker, st: subscription-token) -> result<_, error>\n}";
const _: &str = "/// An interface for a consumer relying on push-based message delivery.\ndefault interface handler {\n    use pkg.types.{event}\n    use err.tracing.{error}\n\n\t/// Creates an on-receive handler push-based message delivery.\n  \ton-receive: func(e: event) -> result<_, error>\n}";
const _: &str = "/// An interface for a producer.\ndefault interface producer { \n    use pkg.types.{broker, channel, event}\n    use err.tracing.{error}\n\n\t/// Publishes an event to a channel in a broker.\n\tpublish: func(b: broker, c: channel, e: event) -> result<_, error>\n}";
const _: &str = "/// An interface grouping all necessary types for messaging.\ndefault interface messaging-types {\n  use err.tracing.{error}\n\n  /// A broker type that allows the exchange of messages.\n  type broker = u32\n  drop-broker: func(b: broker)\n  open-broker: func(name: string) -> result<broker, error>\n\n  /// An event type that follows the CloudEvents specification (https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md). We\n  /// assume the type of the data is a byte sequence. It is up to the data schema to determine what type of the data payload the event \n  /// contains.\n  record event {\n    specversion: string,\n    ty: string,\n    source: string,\n    id: string,\n    data: option<list<u8>>,\n    datacontenttype: option<string>,\n    dataschema: option<string>,\n    subject: option<string>,\n    time: option<string>,\t\n    extensions: option<list<tuple<string, string>>>\n  }\n    \n  /// Channels specify where a published message should land. There are two types of channels:\n  /// - queue: competitive consumers, and\n  /// - topic: non-competitive consumers.\n  variant channel {\n    queue(string),\n    topic(string)\n  }\n\n  /// A subscription token that allows receives from a specific subscription\n  type subscription-token = string\n}";
struct MyMessaging;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello, world!\n"], &[]));
    };
}
