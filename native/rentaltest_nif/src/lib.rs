#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use]
extern crate rental;

use std::sync::Mutex;

use rustler::{Env, Term, NifResult, Encoder};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.RentalTest.Nif",
    [
        ("add", 2, add),
        ("start_thread", 0, start_thread),
        ("some_call", 1, some_call),
        ("exit_thread", 1, exit_thread)
    ],
    Some(on_load)
}

// Rental example

rental! {
    pub mod testing {
        #[rental]
        pub struct Testing {
            a: String,
            b: &'a str,
        }
    }
}

struct TestResource {
    inner: Mutex<testing::Testing>,
}

pub fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    rustler::resource_struct_init!(TestResource, env);
    rustler::resource_struct_init!(ChannelResourceWrapper, env);
    true
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}


//
// Channel example
//


use std::sync::mpsc::{ channel, Sender, Receiver };
use ::rustler::resource::ResourceArc;

struct ChannelResourceWrapper(Mutex<ChannelResource>);
struct ChannelResource {
    sender: Sender<CallMessage>,
    receiver: Receiver<ReturnMessage>,
}

enum CallMessage {
    SomeCall,
    Exit,
}

#[derive(Eq, PartialEq)]
enum ReturnMessage {
    SomeReturn,
    ExitOk,
}

fn thread_fun(call_recv: Receiver<CallMessage>, resp_send: Sender<ReturnMessage>) {
    loop {
        let call = call_recv.recv().unwrap();
        match call {
            CallMessage::SomeCall => {
                resp_send.send(ReturnMessage::SomeReturn).unwrap();
            },
            CallMessage::Exit => {
                resp_send.send(ReturnMessage::ExitOk).unwrap();
                return;
            },
        }
    }
}

fn start_thread<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let (call_send, call_recv) = channel::<CallMessage>();
    let (resp_send, resp_recv) = channel::<ReturnMessage>();
    ::std::thread::spawn(move || {
        thread_fun(call_recv, resp_send);
    });
    let res = ChannelResource {
        sender: call_send,
        receiver: resp_recv,
    };
    let wrapper = ChannelResourceWrapper(Mutex::new(res));

    Ok((atoms::ok(), ResourceArc::new(wrapper)).encode(env))
}

fn some_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<ChannelResourceWrapper> = args[0].decode()?;
    let lock = res.0.lock().unwrap();
    lock.sender.send(CallMessage::SomeCall).unwrap();
    assert!(lock.receiver.recv().unwrap() == ReturnMessage::SomeReturn);
    Ok(atoms::ok().encode(env))
}

fn exit_thread<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let res: ResourceArc<ChannelResourceWrapper> = args[0].decode()?;
    let lock = res.0.lock().unwrap();
    lock.sender.send(CallMessage::Exit).unwrap();
    assert!(lock.receiver.recv().unwrap() == ReturnMessage::ExitOk);
    Ok(atoms::ok().encode(env))
}






