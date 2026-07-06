_oura() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="oura"
                ;;
            oura,activity)
                cmd="oura__subcmd__activity"
                ;;
            oura,api)
                cmd="oura__subcmd__api"
                ;;
            oura,auth)
                cmd="oura__subcmd__auth"
                ;;
            oura,completion)
                cmd="oura__subcmd__completion"
                ;;
            oura,heartrate)
                cmd="oura__subcmd__heartrate"
                ;;
            oura,help)
                cmd="oura__subcmd__help"
                ;;
            oura,man)
                cmd="oura__subcmd__man"
                ;;
            oura,mcp)
                cmd="oura__subcmd__mcp"
                ;;
            oura,personal-info)
                cmd="oura__subcmd__personal__subcmd__info"
                ;;
            oura,readiness)
                cmd="oura__subcmd__readiness"
                ;;
            oura,sessions)
                cmd="oura__subcmd__sessions"
                ;;
            oura,sleep)
                cmd="oura__subcmd__sleep"
                ;;
            oura,stress)
                cmd="oura__subcmd__stress"
                ;;
            oura,workouts)
                cmd="oura__subcmd__workouts"
                ;;
            oura__subcmd__auth,help)
                cmd="oura__subcmd__auth__subcmd__help"
                ;;
            oura__subcmd__auth,login)
                cmd="oura__subcmd__auth__subcmd__login"
                ;;
            oura__subcmd__auth,logout)
                cmd="oura__subcmd__auth__subcmd__logout"
                ;;
            oura__subcmd__auth,refresh)
                cmd="oura__subcmd__auth__subcmd__refresh"
                ;;
            oura__subcmd__auth,setup)
                cmd="oura__subcmd__auth__subcmd__setup"
                ;;
            oura__subcmd__auth,status)
                cmd="oura__subcmd__auth__subcmd__status"
                ;;
            oura__subcmd__auth,token)
                cmd="oura__subcmd__auth__subcmd__token"
                ;;
            oura__subcmd__auth__subcmd__help,help)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__help"
                ;;
            oura__subcmd__auth__subcmd__help,login)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__login"
                ;;
            oura__subcmd__auth__subcmd__help,logout)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__logout"
                ;;
            oura__subcmd__auth__subcmd__help,refresh)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__refresh"
                ;;
            oura__subcmd__auth__subcmd__help,setup)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__setup"
                ;;
            oura__subcmd__auth__subcmd__help,status)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__status"
                ;;
            oura__subcmd__auth__subcmd__help,token)
                cmd="oura__subcmd__auth__subcmd__help__subcmd__token"
                ;;
            oura__subcmd__help,activity)
                cmd="oura__subcmd__help__subcmd__activity"
                ;;
            oura__subcmd__help,api)
                cmd="oura__subcmd__help__subcmd__api"
                ;;
            oura__subcmd__help,auth)
                cmd="oura__subcmd__help__subcmd__auth"
                ;;
            oura__subcmd__help,completion)
                cmd="oura__subcmd__help__subcmd__completion"
                ;;
            oura__subcmd__help,heartrate)
                cmd="oura__subcmd__help__subcmd__heartrate"
                ;;
            oura__subcmd__help,help)
                cmd="oura__subcmd__help__subcmd__help"
                ;;
            oura__subcmd__help,man)
                cmd="oura__subcmd__help__subcmd__man"
                ;;
            oura__subcmd__help,mcp)
                cmd="oura__subcmd__help__subcmd__mcp"
                ;;
            oura__subcmd__help,personal-info)
                cmd="oura__subcmd__help__subcmd__personal__subcmd__info"
                ;;
            oura__subcmd__help,readiness)
                cmd="oura__subcmd__help__subcmd__readiness"
                ;;
            oura__subcmd__help,sessions)
                cmd="oura__subcmd__help__subcmd__sessions"
                ;;
            oura__subcmd__help,sleep)
                cmd="oura__subcmd__help__subcmd__sleep"
                ;;
            oura__subcmd__help,stress)
                cmd="oura__subcmd__help__subcmd__stress"
                ;;
            oura__subcmd__help,workouts)
                cmd="oura__subcmd__help__subcmd__workouts"
                ;;
            oura__subcmd__help__subcmd__auth,login)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__login"
                ;;
            oura__subcmd__help__subcmd__auth,logout)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__logout"
                ;;
            oura__subcmd__help__subcmd__auth,refresh)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__refresh"
                ;;
            oura__subcmd__help__subcmd__auth,setup)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__setup"
                ;;
            oura__subcmd__help__subcmd__auth,status)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__status"
                ;;
            oura__subcmd__help__subcmd__auth,token)
                cmd="oura__subcmd__help__subcmd__auth__subcmd__token"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        oura)
            opts="-h -V --json --no-color --help --version auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__activity)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__api)
            opts="-X -f -h --method --field --paginate --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --method)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -X)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --field)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth)
            opts="-h --json --no-color --help setup login status logout refresh token help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help)
            opts="setup login status logout refresh token help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__login)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__logout)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__refresh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__help__subcmd__token)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__login)
            opts="-h --port --no-browser --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__logout)
            opts="-h --all --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__refresh)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__setup)
            opts="-h --port --no-browser --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__status)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__auth__subcmd__token)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__completion)
            opts="-h --json --no-color --help bash elvish fish powershell zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__heartrate)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help)
            opts="auth sleep readiness activity stress heartrate sessions workouts personal-info api mcp completion man help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__activity)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__api)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth)
            opts="setup login status logout refresh token"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__login)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__logout)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__refresh)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__auth__subcmd__token)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__completion)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__heartrate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__man)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__mcp)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__personal__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__readiness)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__sessions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__sleep)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__stress)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__help__subcmd__workouts)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__man)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__mcp)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__personal__subcmd__info)
            opts="-h --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__readiness)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__sessions)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__sleep)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__stress)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        oura__subcmd__workouts)
            opts="-h --start --end --date --json --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --start)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --end)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _oura -o nosort -o bashdefault -o default oura
else
    complete -F _oura -o bashdefault -o default oura
fi
