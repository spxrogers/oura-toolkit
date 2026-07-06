#compdef oura

autoload -U is-at-least

_oura() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_oura_commands" \
"*::: :->oura" \
&& ret=0
    case $state in
    (oura)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:oura-command-$line[1]:"
        case $line[1] in
            (auth)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_oura__subcmd__auth_commands" \
"*::: :->auth" \
&& ret=0

    case $state in
    (auth)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:oura-auth-command-$line[1]:"
        case $line[1] in
            (setup)
_arguments "${_arguments_options[@]}" : \
'--port=[Loopback port for the redirect URI (must match your registered app)]:PORT:_default' \
'--no-browser[Skip the local browser+loopback\: print the URL and paste the redirect back (for SSH/containers where the callback can'\''t reach this host)]' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(login)
_arguments "${_arguments_options[@]}" : \
'--port=[Loopback port for the redirect URI (must match your registered app)]:PORT:_default' \
'--no-browser[Skip the local browser+loopback\: print the URL and paste the redirect back (for SSH/containers where the callback can'\''t reach this host)]' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(logout)
_arguments "${_arguments_options[@]}" : \
'--all[Also remove the stored client credentials (client_id + client_secret)]' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(refresh)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(token)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_oura__subcmd__auth__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:oura-auth-help-command-$line[1]:"
        case $line[1] in
            (setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(login)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(logout)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(refresh)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(token)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(sleep)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(readiness)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(activity)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stress)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(heartrate)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sessions)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(workouts)
_arguments "${_arguments_options[@]}" : \
'--start=[Start date\: today, yesterday, or YYYY-MM-DD (default\: 6 days before --end)]:START:_default' \
'--end=[End date\: today, yesterday, or YYYY-MM-DD (default\: today)]:END:_default' \
'--date=[A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end]:DATE:_default' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(personal-info)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(api)
_arguments "${_arguments_options[@]}" : \
'-X+[HTTP method (default GET)]:METHOD:_default' \
'--method=[HTTP method (default GET)]:METHOD:_default' \
'*-f+[Add a field (key=value). Query param for GET/HEAD/DELETE, else a JSON body field. Repeatable]:FIELD:_default' \
'*--field=[Add a field (key=value). Query param for GET/HEAD/DELETE, else a JSON body field. Repeatable]:FIELD:_default' \
'--paginate[Follow \`next_token\` pagination and aggregate every page'\''s \`data\` array into one \`{"data"\:\[…\]}\` object (GET only)]' \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
':path -- Request path, resolved against the API base URL (e.g. /v2/usercollection/personal_info). A leading `/` is optional:_default' \
&& ret=0
;;
(mcp)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
':shell -- Shell to generate the completion script for:(bash elvish fish powershell zsh)' \
&& ret=0
;;
(man)
_arguments "${_arguments_options[@]}" : \
'--json[Output JSON instead of the default table/plain rendering (data commands and \`auth status\`)]' \
'--no-color[Disable colored output (also honored\: the NO_COLOR env var)]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_oura__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:oura-help-command-$line[1]:"
        case $line[1] in
            (auth)
_arguments "${_arguments_options[@]}" : \
":: :_oura__subcmd__help__subcmd__auth_commands" \
"*::: :->auth" \
&& ret=0

    case $state in
    (auth)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:oura-help-auth-command-$line[1]:"
        case $line[1] in
            (setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(login)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(logout)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(refresh)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(token)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(sleep)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(readiness)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(activity)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(heartrate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sessions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(workouts)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(personal-info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(api)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mcp)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(man)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_oura_commands] )) ||
_oura_commands() {
    local commands; commands=(
'auth:Authentication (OAuth) flows' \
'sleep:Daily sleep summaries (score + contributors)' \
'readiness:Daily readiness summaries' \
'activity:Daily activity summaries (score, steps, calories)' \
'stress:Daily stress summaries' \
'heartrate:Heart-rate time series (frequent bpm samples)' \
'sessions:Moment/session records (meditation, naps, …)' \
'workouts:Workout records' \
'personal-info:Your Oura profile (age, height, weight, …)' \
'api:Authenticated passthrough to an arbitrary Oura API endpoint (like \`gh api\`)' \
'mcp:Run as a STDIO MCP server (8 read-only Oura data tools)' \
'completion:Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)' \
'man:Print the \`oura\` man page (roff) to stdout' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'oura commands' commands "$@"
}
(( $+functions[_oura__subcmd__activity_commands] )) ||
_oura__subcmd__activity_commands() {
    local commands; commands=()
    _describe -t commands 'oura activity commands' commands "$@"
}
(( $+functions[_oura__subcmd__api_commands] )) ||
_oura__subcmd__api_commands() {
    local commands; commands=()
    _describe -t commands 'oura api commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth_commands] )) ||
_oura__subcmd__auth_commands() {
    local commands; commands=(
'setup:Guided Oura OAuth app registration (terminal prompts), then login' \
'login:Authorization Code login using stored client credentials' \
'status:Show stored auth state\: client_id, scopes, token expiry' \
'logout:Delete stored tokens (log out). Keeps the client credentials unless --all is given' \
'refresh:Force a token refresh now and persist the rotated refresh token' \
'token:Print a valid access token (refreshing if needed) to stdout — and nothing else' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'oura auth commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help_commands] )) ||
_oura__subcmd__auth__subcmd__help_commands() {
    local commands; commands=(
'setup:Guided Oura OAuth app registration (terminal prompts), then login' \
'login:Authorization Code login using stored client credentials' \
'status:Show stored auth state\: client_id, scopes, token expiry' \
'logout:Delete stored tokens (log out). Keeps the client credentials unless --all is given' \
'refresh:Force a token refresh now and persist the rotated refresh token' \
'token:Print a valid access token (refreshing if needed) to stdout — and nothing else' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'oura auth help commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__help_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help help commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__login_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__login_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help login commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__logout_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__logout_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help logout commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__refresh_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__refresh_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help refresh commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__setup_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help setup commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__status_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help status commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__help__subcmd__token_commands] )) ||
_oura__subcmd__auth__subcmd__help__subcmd__token_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth help token commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__login_commands] )) ||
_oura__subcmd__auth__subcmd__login_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth login commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__logout_commands] )) ||
_oura__subcmd__auth__subcmd__logout_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth logout commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__refresh_commands] )) ||
_oura__subcmd__auth__subcmd__refresh_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth refresh commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__setup_commands] )) ||
_oura__subcmd__auth__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth setup commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__status_commands] )) ||
_oura__subcmd__auth__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth status commands' commands "$@"
}
(( $+functions[_oura__subcmd__auth__subcmd__token_commands] )) ||
_oura__subcmd__auth__subcmd__token_commands() {
    local commands; commands=()
    _describe -t commands 'oura auth token commands' commands "$@"
}
(( $+functions[_oura__subcmd__completion_commands] )) ||
_oura__subcmd__completion_commands() {
    local commands; commands=()
    _describe -t commands 'oura completion commands' commands "$@"
}
(( $+functions[_oura__subcmd__heartrate_commands] )) ||
_oura__subcmd__heartrate_commands() {
    local commands; commands=()
    _describe -t commands 'oura heartrate commands' commands "$@"
}
(( $+functions[_oura__subcmd__help_commands] )) ||
_oura__subcmd__help_commands() {
    local commands; commands=(
'auth:Authentication (OAuth) flows' \
'sleep:Daily sleep summaries (score + contributors)' \
'readiness:Daily readiness summaries' \
'activity:Daily activity summaries (score, steps, calories)' \
'stress:Daily stress summaries' \
'heartrate:Heart-rate time series (frequent bpm samples)' \
'sessions:Moment/session records (meditation, naps, …)' \
'workouts:Workout records' \
'personal-info:Your Oura profile (age, height, weight, …)' \
'api:Authenticated passthrough to an arbitrary Oura API endpoint (like \`gh api\`)' \
'mcp:Run as a STDIO MCP server (8 read-only Oura data tools)' \
'completion:Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)' \
'man:Print the \`oura\` man page (roff) to stdout' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'oura help commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__activity_commands] )) ||
_oura__subcmd__help__subcmd__activity_commands() {
    local commands; commands=()
    _describe -t commands 'oura help activity commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__api_commands] )) ||
_oura__subcmd__help__subcmd__api_commands() {
    local commands; commands=()
    _describe -t commands 'oura help api commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth_commands] )) ||
_oura__subcmd__help__subcmd__auth_commands() {
    local commands; commands=(
'setup:Guided Oura OAuth app registration (terminal prompts), then login' \
'login:Authorization Code login using stored client credentials' \
'status:Show stored auth state\: client_id, scopes, token expiry' \
'logout:Delete stored tokens (log out). Keeps the client credentials unless --all is given' \
'refresh:Force a token refresh now and persist the rotated refresh token' \
'token:Print a valid access token (refreshing if needed) to stdout — and nothing else' \
    )
    _describe -t commands 'oura help auth commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__login_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__login_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth login commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__logout_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__logout_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth logout commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__refresh_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__refresh_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth refresh commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__setup_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth setup commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__status_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth status commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__auth__subcmd__token_commands] )) ||
_oura__subcmd__help__subcmd__auth__subcmd__token_commands() {
    local commands; commands=()
    _describe -t commands 'oura help auth token commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__completion_commands] )) ||
_oura__subcmd__help__subcmd__completion_commands() {
    local commands; commands=()
    _describe -t commands 'oura help completion commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__heartrate_commands] )) ||
_oura__subcmd__help__subcmd__heartrate_commands() {
    local commands; commands=()
    _describe -t commands 'oura help heartrate commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__help_commands] )) ||
_oura__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'oura help help commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__man_commands] )) ||
_oura__subcmd__help__subcmd__man_commands() {
    local commands; commands=()
    _describe -t commands 'oura help man commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__mcp_commands] )) ||
_oura__subcmd__help__subcmd__mcp_commands() {
    local commands; commands=()
    _describe -t commands 'oura help mcp commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__personal-info_commands] )) ||
_oura__subcmd__help__subcmd__personal-info_commands() {
    local commands; commands=()
    _describe -t commands 'oura help personal-info commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__readiness_commands] )) ||
_oura__subcmd__help__subcmd__readiness_commands() {
    local commands; commands=()
    _describe -t commands 'oura help readiness commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__sessions_commands] )) ||
_oura__subcmd__help__subcmd__sessions_commands() {
    local commands; commands=()
    _describe -t commands 'oura help sessions commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__sleep_commands] )) ||
_oura__subcmd__help__subcmd__sleep_commands() {
    local commands; commands=()
    _describe -t commands 'oura help sleep commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__stress_commands] )) ||
_oura__subcmd__help__subcmd__stress_commands() {
    local commands; commands=()
    _describe -t commands 'oura help stress commands' commands "$@"
}
(( $+functions[_oura__subcmd__help__subcmd__workouts_commands] )) ||
_oura__subcmd__help__subcmd__workouts_commands() {
    local commands; commands=()
    _describe -t commands 'oura help workouts commands' commands "$@"
}
(( $+functions[_oura__subcmd__man_commands] )) ||
_oura__subcmd__man_commands() {
    local commands; commands=()
    _describe -t commands 'oura man commands' commands "$@"
}
(( $+functions[_oura__subcmd__mcp_commands] )) ||
_oura__subcmd__mcp_commands() {
    local commands; commands=()
    _describe -t commands 'oura mcp commands' commands "$@"
}
(( $+functions[_oura__subcmd__personal-info_commands] )) ||
_oura__subcmd__personal-info_commands() {
    local commands; commands=()
    _describe -t commands 'oura personal-info commands' commands "$@"
}
(( $+functions[_oura__subcmd__readiness_commands] )) ||
_oura__subcmd__readiness_commands() {
    local commands; commands=()
    _describe -t commands 'oura readiness commands' commands "$@"
}
(( $+functions[_oura__subcmd__sessions_commands] )) ||
_oura__subcmd__sessions_commands() {
    local commands; commands=()
    _describe -t commands 'oura sessions commands' commands "$@"
}
(( $+functions[_oura__subcmd__sleep_commands] )) ||
_oura__subcmd__sleep_commands() {
    local commands; commands=()
    _describe -t commands 'oura sleep commands' commands "$@"
}
(( $+functions[_oura__subcmd__stress_commands] )) ||
_oura__subcmd__stress_commands() {
    local commands; commands=()
    _describe -t commands 'oura stress commands' commands "$@"
}
(( $+functions[_oura__subcmd__workouts_commands] )) ||
_oura__subcmd__workouts_commands() {
    local commands; commands=()
    _describe -t commands 'oura workouts commands' commands "$@"
}

if [ "$funcstack[1]" = "_oura" ]; then
    _oura "$@"
else
    compdef _oura oura
fi
