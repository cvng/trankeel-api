# Contributing

## Release flow

**During this process [`main`](https://github.com/piteo-team/piteo/tree/main)
branch (or any other branch that you're creating release from) should be frozen
and no commits should land until the release is cut.**

### Updating `piteo`

1. Create a PR that bumps versions of all packages in the
   [`packages`](https://github.com/piteo-team/piteo/tree/main/packages) and
   [`piteo`](https://github.com/piteo-team/piteo/tree/main) directories.

To help you with this, you can use
[`bump.sh`](https://github.com/piteo-team/piteo/blob/main/tools/bump.sh).

When in doubt always do a minor bump instead of a patch. In essentially every
release all packages will need a minor bump. Patch bumps are the exception, not
the norm.

2. Update `CHANGELOG.md` if necessary.

3. Make sure CI pipeline passes.

4. Deploy [`piteo-api`][piteo-api] app to [`heroku.com`][piteo-api-heroku].

**Make sure that [`heroku`][heroku-cli] is logged on with a user that has
permissions to deploy those apps.**

This is done by running `heroku deploy:<pr-branch> -a piteo-api`.

If there are any problems when you deploy, that require you to change the code,
then after applying the fixes they should be commited and pushed to the PR.

In the event you deploy bad code, use `heroku rollback` to restore the previous
release.

5. Deploy [`piteo-web`][piteo-web] app to [`heroku.com`][piteo-web-heroku].

This is done by running `heroku deploy:<pr-branch> -a piteo-web`.

6. Once all apps are deployed merge the PR.

7. Create a tag with the version number (_without_ `v` prefix).

8. Wait for CI pipeline on the created tag branch to pass.

The CI pipeline will create a release draft on GitHub
(https://github.com/piteo-team/piteo/releases).

9. Publish the release on Github.

10. Share the news with the Piteo team on the Slack channel
    https://piteo-team.slack.com.

[piteo-api]: https://github.com/piteo-team/piteo/tree/main/packages/api
[piteo-web]: https://github.com/piteo-team/piteo/tree/main/packages/web
[piteo-api-heroku]: https://dashboard.heroku.com/apps/piteo-api
[piteo-web-heroku]: https://dashboard.heroku.com/apps/piteo-web
[heroku-cli]: https://devcenter.heroku.com/articles/heroku-cli
