//! Implements a command for managing projects.
use clap::{App, AppSettings, ArgMatches};
use failure::Error;

use api::Api;
use config::Config;
use utils::args::ArgExt;
use utils::formatting::Table;

pub fn make_app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Manage projects on Sentry.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .org_arg()
        .subcommand(App::new("list").about("List all projects for an organization."))
}

pub fn execute<'a>(matches: &ArgMatches<'a>) -> Result<(), Error> {
    let config = Config::get_current();
    let api = Api::get_current();
    let org = config.get_org(matches)?;
    let mut projects = api.list_organization_projects(&org)?;
    projects.sort_by_key(|p| (p.team.name.clone(), p.name.clone()));

    let mut table = Table::new();
    table.title_row().add("Slug").add("Team").add("Name");

    for project in &projects {
        table
            .add_row()
            .add(&project.slug)
            .add(&project.team.name)
            .add(&project.name);
    }

    table.print();

    Ok(())
}
