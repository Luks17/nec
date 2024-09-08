// use genco::{quote, Tokens};
// use nec::{core::{
//     out::write_file,
//     react::{self, React},
//     FileKind,
// }, setup::init};

use anyhow::Result;
use nec::setup::init;

fn main() -> Result<()> {
    // let use_state = &react::import("react", "useState");
    // let app_tks: Tokens<React> = quote! {
    //     export default function Page() {
    //         const [count, setCount] = $use_state(0);
    //
    //         return <>
    //             <h1>Hello $("a")!</h1>
    //
    //             <button onClick={() => setCount((count) => count + 1)}>
    //                 Count is {count}
    //             </button>
    //         </>
    //     }
    // };
    //
    // write_file("page.tsx", app_tks, Some(FileKind::ClientComponent));

    init()?;

    Ok(())
}
