# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_oura_global_optspecs
    string join \n json no-color h/help V/version
end

function __fish_oura_needs_command
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s (__fish_oura_global_optspecs) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function __fish_oura_using_subcommand
    set -l cmd (__fish_oura_needs_command)
    test -z "$cmd"
    and return 1
    contains -- $cmd[1] $argv
end

complete -c oura -n "__fish_oura_needs_command" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_needs_command" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_needs_command" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_needs_command" -s V -l version -d 'Print version'
complete -c oura -n "__fish_oura_needs_command" -f -a "auth" -d 'Authentication (OAuth) flows'
complete -c oura -n "__fish_oura_needs_command" -f -a "sleep" -d 'Daily sleep summaries (score + contributors)'
complete -c oura -n "__fish_oura_needs_command" -f -a "readiness" -d 'Daily readiness summaries'
complete -c oura -n "__fish_oura_needs_command" -f -a "activity" -d 'Daily activity summaries (score, steps, calories)'
complete -c oura -n "__fish_oura_needs_command" -f -a "stress" -d 'Daily stress summaries'
complete -c oura -n "__fish_oura_needs_command" -f -a "heartrate" -d 'Heart-rate time series (frequent bpm samples)'
complete -c oura -n "__fish_oura_needs_command" -f -a "sessions" -d 'Moment/session records (meditation, naps, …)'
complete -c oura -n "__fish_oura_needs_command" -f -a "workouts" -d 'Workout records'
complete -c oura -n "__fish_oura_needs_command" -f -a "personal-info" -d 'Your Oura profile (age, height, weight, …)'
complete -c oura -n "__fish_oura_needs_command" -f -a "api" -d 'Authenticated passthrough to an arbitrary Oura API endpoint (like `gh api`)'
complete -c oura -n "__fish_oura_needs_command" -f -a "mcp" -d 'Run as a STDIO MCP server (8 read-only Oura data tools)'
complete -c oura -n "__fish_oura_needs_command" -f -a "completion" -d 'Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)'
complete -c oura -n "__fish_oura_needs_command" -f -a "man" -d 'Print the `oura` man page (roff) to stdout'
complete -c oura -n "__fish_oura_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "setup" -d 'Guided Oura OAuth app registration (terminal prompts), then login'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "login" -d 'Authorization Code login using stored client credentials'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "status" -d 'Show stored auth state: client_id, scopes, token expiry'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "logout" -d 'Delete stored tokens (log out). Keeps the client credentials unless --all is given'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "refresh" -d 'Force a token refresh now and persist the rotated refresh token'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "token" -d 'Print a valid access token (refreshing if needed) to stdout — and nothing else'
complete -c oura -n "__fish_oura_using_subcommand auth; and not __fish_seen_subcommand_from setup login status logout refresh token help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from setup" -l port -d 'Loopback port for the redirect URI (must match your registered app)' -r
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from setup" -l no-browser -d 'Skip the local browser+loopback: print the URL and paste the redirect back (for SSH/containers where the callback can\'t reach this host)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from setup" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from setup" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from setup" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from login" -l port -d 'Loopback port for the redirect URI (must match your registered app)' -r
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from login" -l no-browser -d 'Skip the local browser+loopback: print the URL and paste the redirect back (for SSH/containers where the callback can\'t reach this host)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from login" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from login" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from login" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from status" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from status" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from status" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from logout" -l all -d 'Also remove the stored client credentials (client_id + client_secret)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from logout" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from logout" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from logout" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from refresh" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from refresh" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from refresh" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from token" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from token" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from token" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "setup" -d 'Guided Oura OAuth app registration (terminal prompts), then login'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "login" -d 'Authorization Code login using stored client credentials'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "status" -d 'Show stored auth state: client_id, scopes, token expiry'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "logout" -d 'Delete stored tokens (log out). Keeps the client credentials unless --all is given'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "refresh" -d 'Force a token refresh now and persist the rotated refresh token'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "token" -d 'Print a valid access token (refreshing if needed) to stdout — and nothing else'
complete -c oura -n "__fish_oura_using_subcommand auth; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c oura -n "__fish_oura_using_subcommand sleep" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand sleep" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand sleep" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand sleep" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand sleep" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand sleep" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand readiness" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand readiness" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand readiness" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand readiness" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand readiness" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand readiness" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand activity" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand activity" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand activity" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand activity" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand activity" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand activity" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand stress" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand stress" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand stress" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand stress" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand stress" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand stress" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand heartrate" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand heartrate" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand heartrate" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand heartrate" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand heartrate" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand heartrate" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand sessions" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand sessions" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand sessions" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand sessions" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand sessions" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand sessions" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand workouts" -l start -d 'Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)' -r
complete -c oura -n "__fish_oura_using_subcommand workouts" -l end -d 'End date: today, yesterday, or YYYY-MM-DD (default: today)' -r
complete -c oura -n "__fish_oura_using_subcommand workouts" -l date -d 'A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end' -r
complete -c oura -n "__fish_oura_using_subcommand workouts" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand workouts" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand workouts" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand personal-info" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand personal-info" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand personal-info" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand api" -s X -l method -d 'HTTP method (default GET)' -r
complete -c oura -n "__fish_oura_using_subcommand api" -s f -l field -d 'Add a field (key=value). Query param for GET/HEAD/DELETE, else a JSON body field. Repeatable' -r
complete -c oura -n "__fish_oura_using_subcommand api" -l paginate -d 'Follow `next_token` pagination and aggregate every page\'s `data` array into one `{"data":[…]}` object (GET only)'
complete -c oura -n "__fish_oura_using_subcommand api" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand api" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand api" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c oura -n "__fish_oura_using_subcommand mcp" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand mcp" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand mcp" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand completion" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand completion" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand completion" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand man" -l json -d 'Output JSON instead of the default table/plain rendering (data commands and `auth status`)'
complete -c oura -n "__fish_oura_using_subcommand man" -l no-color -d 'Disable colored output (also honored: the NO_COLOR env var)'
complete -c oura -n "__fish_oura_using_subcommand man" -s h -l help -d 'Print help'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "auth" -d 'Authentication (OAuth) flows'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "sleep" -d 'Daily sleep summaries (score + contributors)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "readiness" -d 'Daily readiness summaries'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "activity" -d 'Daily activity summaries (score, steps, calories)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "stress" -d 'Daily stress summaries'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "heartrate" -d 'Heart-rate time series (frequent bpm samples)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "sessions" -d 'Moment/session records (meditation, naps, …)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "workouts" -d 'Workout records'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "personal-info" -d 'Your Oura profile (age, height, weight, …)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "api" -d 'Authenticated passthrough to an arbitrary Oura API endpoint (like `gh api`)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "mcp" -d 'Run as a STDIO MCP server (8 read-only Oura data tools)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "completion" -d 'Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "man" -d 'Print the `oura` man page (roff) to stdout'
complete -c oura -n "__fish_oura_using_subcommand help; and not __fish_seen_subcommand_from auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "setup" -d 'Guided Oura OAuth app registration (terminal prompts), then login'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "login" -d 'Authorization Code login using stored client credentials'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "status" -d 'Show stored auth state: client_id, scopes, token expiry'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "logout" -d 'Delete stored tokens (log out). Keeps the client credentials unless --all is given'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "refresh" -d 'Force a token refresh now and persist the rotated refresh token'
complete -c oura -n "__fish_oura_using_subcommand help; and __fish_seen_subcommand_from auth" -f -a "token" -d 'Print a valid access token (refreshing if needed) to stdout — and nothing else'
