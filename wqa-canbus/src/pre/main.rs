use super::error::CanError;
use super::protocol::*;
pub use rhal::Engine;

#[derive(Debug, StructOpt, Clone)]
pub enum Command {

    #[structopt(name = "read")]
    /// Create a new Review Proof from reviewed list
    Read(Commit),

    #[structopt(name = "read")]
    /// Create a new Review Proof from reviewed list
    Read(Commit),


    #[structopt(name = "package")]
    /// Package settings
    Package(Package),

    #[structopt(name = "status")]
    /// Display pending review list
    Status,

    #[structopt(name = "rm")]
    /// Remove path from reviewed list
    Remove(Remove),

    #[structopt(name = "verify")]
    /// Verify review coverage of the package
    Verify(Verify),

    #[structopt(name = "trust")]
    /// Trust Store management
    Trust(Trust),

    #[structopt(name = "db")]
    /// Trust Store
    Db(Db),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "pcan", about = "Can system")]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
    //    #[structopt(flatten)]
    //    verbosity: Verbosity,
}

fn main() -> CliResult {

    
    let mut engine = Engine::new();

}

#[derive(Debug, StructOpt, Clone)]
pub struct Msg {
    #[structopt(long = "node")]
    node: u32,
    #[structopt(long = "index")]
    index: u64,
    #[structopt(long = "subindex")]
    subinex: u64,
    #[structopt(long = "low-cost")]
    low_cost: u64,
    #[structopt(long = "allow-dirty")]
    pub allow_dirty: bool,
}



#[derive(Debug, StructOpt, Clone)]
pub enum Command {

    #[structopt(name = "Message")]
    /// Create a new Review Proof from reviewed list
    Message(msg),

    #[structopt(name = "package")]
    /// Package settings
    E(ngenePackage),

    #[structopt(name = "status")]
    /// Display pending review list
    Status,

}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "crev", about = "Distributed code review system")]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
    //    #[structopt(flatten)]
    //    verbosity: Verbosity,
}


main!(|opts: opts::Opts| match opts.command {
    opts::Command::Message(msg) => {
        let mut repo = Repo::auto_open()?;
        if opts.all {
        } else {
            repo.commit(&crev_common::read_passphrase, opts.allow_dirty)?;
        }
    }
    opts::Command::Server(server) => match package {
        opts::Package::Init => {
            let local = Local::auto_open()?;
            let cur_id = local.read_current_id()?;
            Repo::init(&PathBuf::from(".".to_string()), cur_id.to_string())?;
        }
        opts::Package::Trust(package_trust) => {
            let mut repo = Repo::auto_open()?;
            repo.trust_package(&crev_common::read_passphrase, package_trust.allow_dirty)?;
        }
        opts::Package::Verify(args) => {
            let mut repo = Repo::auto_open()?;
            let local = Local::auto_create_or_open()?;
            println!(
                "{}",
                repo.package_verify(&local, args.allow_dirty, args.for_id, &args.trust_params)?
            );
        }
        opts::Package::Digest(digest) => {
            let mut repo = Repo::auto_open()?;
            println!("{}", repo.package_digest(digest.allow_dirty)?);
        }
    },
    opts::Command::Status => {
        let mut repo = Repo::auto_open()?;
        repo.status()?;
    }
    opts::Command::Remove(remove) => {
        let mut repo = Repo::auto_open()?;
        repo.remove(remove.paths)?;
    }
    opts::Command::Verify(verify_opts) => {
        let mut repo = Repo::auto_open()?;
        repo.package_verify(verify_opts.allow_dirty)?;
    }
    opts::Command::Db(cmd) => match cmd {
        opts::Db::Git(git) => {
            let local = Local::auto_open()?;
            let status = local.run_git(git.args)?;
            std::process::exit(status.code().unwrap_or(-159));
        }
        opts::Db::Fetch => {
            let local = Local::auto_open()?;
            local.fetch_trusted(default())?;
        }
    },
});
