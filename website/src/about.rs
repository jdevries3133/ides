use crate::prelude::*;

pub async fn about() -> Response {
    Page {
        title: "About",
        children: &PageContainer {
            children: &About {},
        },
    }
    .render()
    .into_response()
}

struct About;
impl Component for About {
    fn render(&self) -> String {
        r#"
        <div class="prose text-slate-100">
            <h1>About the Site</h1>
            <p>
                This is a site for my friends and family to beta-read
                Ides of August!
            </p>
            <p>
                The purpose of this site is to keep my manuscript safe, and to
                allow me to get feedback without sharing full copies of my
                book. This is a team effort - treat the token you use to access
                this site like a password. If you leak your token, you've
                leaked my book! Never share your token. If you think your token
                may have been compromised, let us know and we'll cycle it out
                for a new one.
            </p>
            <p>
                When you're reading the book, use the toolbar at the bottom to
                change your page. If you click on any paragraph, a dialog will
                appear for you to share feedback with me or take notes! Swiping
                to change pages is not supported. Sometimes, the page of text
                may extend past the bottom of the screen, requiring a bit of
                horizonal scrolling. Ideally this wouldn't happen, but the code
                to word count for a given screen size is a bit janky.
            </p>
            <p>
                For any technical questions about this site, you can reach out
                to <a class="link" href="mailto:jdevries3133@gmail.com">
                jdevries3133@gmail.com</a>.
            </p>
            <h1>About Ides of August</h1>
            <p>
                After he officially came to power in 27 BCE, in the aftermath
                of the civil wars that tore Rome apart, Emperor Augustus set
                out to write his memoirs, detailing his achievements and
                processing the historic events that had unfolded around and
                because of him. His stories would go on to inspire countless
                Roman poets, including Virgil, fueling his positive propaganda
                effort and perpetuating the image that’s projected of him even
                today. His perspective would become an important resource for
                William Shakespeare as he crafted his Roman plays, even as
                other characters, like Cleopatra and Marc Antony, took center
                stage. To this day, the memoirs are picked apart, dissected,
                misunderstood and reinterpreted by historians around the world.
                Even though they don’t really exist.
            </p>

            <p>

                The memoirs themselves have long since been lost to time, with
                our certainty of their existence only possible because of
                references other ancient texts make to them. They are forever
                sentenced to footnotes and auxiliary discussions, in the
                memories of the Bard and history textbooks, and while pieces of
                its ideas and words remain, any remnants of physical texts are
                gone forever.

            </p>

            <p>

                The mind reels at what Augustus might have said of the time
                when he was just Gaius Octavius, a boy conquering illness and
                death to impress his great uncle, Julius Caesar. Crossing the
                sea in his teenage years to join his kin in battle, despite the
                adversity of piracy and shipwrecks. Rising the ranks of
                military training until the Ides of March, wherein the north
                star of Caesar is assassinated, and Octavius must take his
                place. What did he believe? Who did he trust? Augustus was a
                real man and leader, who lived a real life, and whose busts and
                statues stand in museums of antiquity and modernity. And yet
                his story is so full of imagination, of gaps in information,
                that it is simply demanding to be written about.

            </p>

            <p>

                My first novel, The Ides of August, is structured as if it is
                Augustus’ lost memoir itself, and as such, aims to show the
                valiant, the terrifying, the hideous, all with the
                embellishment and flare that fiction lends to stories like this
                one. In my writing, I also realized the breadth and size of the
                story I was aiming to tell, so while this 80,000-word
                historical fiction novel could very well stand on its own, I am
                currently envisioning it as a duology, with a sequel in
                progress.

            </p>
        </div>
        "#
        .into()
    }
}
