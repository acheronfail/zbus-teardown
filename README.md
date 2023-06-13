```
❯ cargo run
   Compiling zbus-teardown v0.1.0 (/home/acheronfail/tmp/zbus-teardown)
    Finished dev [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/zbus-teardown`
thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime', /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/zbus-3.13.1/src/abstractions/executor.rs:62:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime', /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/zbus-3.13.1/src/abstractions/executor.rs:62:27
stack backtrace:
   0:     0x556775b8947a - std::backtrace_rs::backtrace::libunwind::trace::ha9053a9a07ca49cb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x556775b8947a - std::backtrace_rs::backtrace::trace_unsynchronized::h9c2852a457ad564e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x556775b8947a - std::sys_common::backtrace::_print_fmt::h457936fbfaa0070f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x556775b8947a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5779d7bf7f70cb0c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x556775bacf1e - core::fmt::write::h5a4baaff1bcd3eb5
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs:1232:17
   5:     0x556775b85ed5 - std::io::Write::write_fmt::h4bc1f301cb9e9cce
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/io/mod.rs:1684:15
   6:     0x556775b89245 - std::sys_common::backtrace::_print::h5fcdc36060f177e8
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x556775b89245 - std::sys_common::backtrace::print::h54ca9458b876c8bf
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x556775b8a9ff - std::panicking::default_hook::{{closure}}::hbe471161c7664ed6
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:271:22
   9:     0x556775b8a73b - std::panicking::default_hook::ha3500da57aa4ac4f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:290:9
  10:     0x556775b8b008 - std::panicking::rust_panic_with_hook::h50c09d000dc561d2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:692:13
  11:     0x556775b8af09 - std::panicking::begin_panic_handler::{{closure}}::h9e2b2176e00e0d9c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:583:13
  12:     0x556775b898e6 - std::sys_common::backtrace::__rust_end_short_backtrace::h5739b8e512c09d02
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:150:18
  13:     0x556775b8ac12 - rust_begin_unwind
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
  14:     0x556775bab013 - core::panicking::panic_fmt::hf33a1475b4dc5c3e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
  15:     0x556775a9642c - core::panicking::panic_display::hd65937d099d56865
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:147:5
  16:     0x556775acc1ce - tokio::runtime::scheduler::Handle::current::h4ca9397cffcaa101
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/scheduler/mod.rs:57:27
  17:     0x556775ac535d - tokio::runtime::handle::Handle::current::h95a119bd37517ac2
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/handle.rs:102:20
  18:     0x556775888c13 - tokio::task::spawn::spawn_inner::h20aa8f2a09b6b93b
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/spawn.rs:184:22
  19:     0x55677588a802 - tokio::task::spawn::spawn::h0b489b7ea863232a
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/spawn.rs:169:13
  20:     0x5567759ef1cd - zbus::abstractions::executor::Executor::spawn::h5197e7cbf5207b31
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/zbus-3.13.1/src/abstractions/executor.rs:62:27
  21:     0x5567759483db - zbus::connection::Connection::queue_remove_match::h3f7c8d5257523979
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/zbus-3.13.1/src/connection.rs:1140:9
  22:     0x5567758973ad - <zbus::message_stream::Inner as core::ops::drop::Drop>::drop::hb3fba41d6e06a556
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/zbus-3.13.1/src/message_stream.rs:272:13
  23:     0x5567759c1d27 - core::ptr::drop_in_place<zbus::message_stream::Inner>::h46c720d692d102f6
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  24:     0x5567759c2b2b - core::ptr::drop_in_place<zbus::message_stream::MessageStream>::h2c4812dac4d22d5c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  25:     0x5567759c60c4 - core::ptr::drop_in_place<core::option::Option<zbus::message_stream::MessageStream>>::h0edf3c74d9dfd383
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  26:     0x5567759b62be - core::ptr::drop_in_place<ordered_stream::join::Join<zbus::message_stream::MessageStream,core::option::Option<zbus::message_stream::MessageStream>>>::h7d357dd7e50db39d
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  27:     0x5567759c1967 - core::ptr::drop_in_place<zbus::proxy::SignalStream>::ha54d9dc0f517f569
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  28:     0x55677576717b - core::ptr::drop_in_place<zbus_teardown::StateChangedStream>::h9e5d79116f13ae8e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  29:     0x556775766921 - core::ptr::drop_in_place<zbus_teardown::async_main::{{closure}}::{{closure}}>::h858724644af5c568
                               at /home/acheronfail/tmp/zbus-teardown/src/main.rs:52:5
  30:     0x556775766c98 - core::ptr::drop_in_place<tokio::runtime::task::core::Stage<zbus_teardown::async_main::{{closure}}::{{closure}}>>::hd72531c8412e74d5
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  31:     0x5567757746ad - tokio::runtime::task::core::Core<T,S>::set_stage::{{closure}}::h3ac7658489f938b0
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/core.rs:277:41
  32:     0x55677577245e - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h17d83e04ed10ac0c
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/loom/std/unsafe_cell.rs:14:9
  33:     0x5567757744a7 - tokio::runtime::task::core::Core<T,S>::set_stage::h87925708bd9c8393
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/core.rs:277:9
  34:     0x556775773f42 - tokio::runtime::task::core::Core<T,S>::drop_future_or_output::h23d22f876d9afcf4
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/core.rs:242:13
  35:     0x55677576baed - tokio::runtime::task::harness::cancel_task::{{closure}}::h5c3f821415b9f86a
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/harness.rs:447:9
  36:     0x556775766783 - core::ops::function::FnOnce::call_once::he1716824ec008fba
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  37:     0x55677577627e - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::ha2ff30cb0702e2f8
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panic/unwind_safe.rs:271:9
  38:     0x5567757719a4 - std::panicking::try::do_call::hf8af341d279a36e1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  39:     0x556775771e2b - __rust_try
  40:     0x55677577109a - std::panicking::try::h3f1f57caf2a28e5d
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  41:     0x55677577011a - std::panic::catch_unwind::h781c74a91dfdad3b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  42:     0x55677576b8cd - tokio::runtime::task::harness::cancel_task::h9d197493ca1f2323
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/harness.rs:446:15
  43:     0x55677576d0bc - tokio::runtime::task::harness::Harness<T,S>::shutdown::h81255470e9579d69
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/harness.rs:241:9
  44:     0x55677576a7eb - tokio::runtime::task::raw::shutdown::h1223706d8896f535
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/raw.rs:295:5
  45:     0x556775adddc8 - tokio::runtime::task::raw::RawTask::shutdown::h92d2cc88adeb1567
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/raw.rs:234:18
  46:     0x556775af1fe2 - tokio::runtime::task::Task<S>::shutdown::heb3ec4f4ce5b9238
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/mod.rs:385:9
  47:     0x556775af1a0e - tokio::runtime::task::list::LocalOwnedTasks<S>::close_and_shutdown_all::h4e48ed27ba785142
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/runtime/task/list.rs:227:13
  48:     0x556775b14b29 - tokio::task::local::LocalState::close_and_shutdown_all::h538d86f0cb8fd40b
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/local.rs:1083:9
  49:     0x556775b13eea - <tokio::task::local::LocalSet as core::ops::drop::Drop>::drop::{{closure}}::h454b3feafbc2176f
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/local.rs:837:17
  50:     0x556775b13e10 - tokio::task::local::LocalSet::with_if_possible::{{closure}}::h2566402647891d2c
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/local.rs:710:13
  51:     0x556775ab8ee2 - std::thread::local::LocalKey<T>::try_with::he39403f25c6415bc
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/thread/local.rs:446:16
  52:     0x556775b13cdc - tokio::task::local::LocalSet::with_if_possible::hb40d9a0cc7698561
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/local.rs:693:19
  53:     0x556775b13e98 - <tokio::task::local::LocalSet as core::ops::drop::Drop>::drop::hb23a1790ca1afc02
                               at /home/acheronfail/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.2/src/task/local.rs:832:9
  54:     0x556775767117 - core::ptr::drop_in_place<tokio::task::local::LocalSet>::ha0ffdd62522c807a
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ptr/mod.rs:490:1
  55:     0x55677576a04b - zbus_teardown::main::h55bd750090abc44d
                               at /home/acheronfail/tmp/zbus-teardown/src/main.rs:28:66
  56:     0x5567757665ab - core::ops::function::FnOnce::call_once::h541deb1af316978a
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  57:     0x556775771f6e - std::sys_common::backtrace::__rust_begin_short_backtrace::h143d2f85399da455
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:134:18
  58:     0x5567757703c1 - std::rt::lang_start::{{closure}}::h732407cbbe98b0d3
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:166:18
  59:     0x556775b8246c - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hd6efcd3bec896f2c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:287:13
  60:     0x556775b8246c - std::panicking::try::do_call::hce04e543bb1f4cbb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  61:     0x556775b8246c - std::panicking::try::h3342dd4e1f680968
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  62:     0x556775b8246c - std::panic::catch_unwind::h148ce1e59ac0cee7
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  63:     0x556775b8246c - std::rt::lang_start_internal::{{closure}}::h25f9dda2057a67fe
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:48
  64:     0x556775b8246c - std::panicking::try::do_call::h7caaaeaf9401650b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  65:     0x556775b8246c - std::panicking::try::he7d15285746cbbc2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  66:     0x556775b8246c - std::panic::catch_unwind::h89fb4f50c0301fe0
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  67:     0x556775b8246c - std::rt::lang_start_internal::h078acd489417d3c1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:20
  68:     0x55677577039a - std::rt::lang_start::h1f26f4ed1bb50a8b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:165:17
  69:     0x55677576a26e - main
  70:     0x7ff6c5ada850 - <unknown>
  71:     0x7ff6c5ada90a - __libc_start_main
  72:     0x556775765695 - _start
  73:                0x0 - <unknown>
thread panicked while panicking. aborting.
[1]    10184 IOT instruction (core dumped)  cargo run
```