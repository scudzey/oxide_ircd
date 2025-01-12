#[allow(non_camel_case_types, clippy::zero_prefixed_literal)]
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    RPL_WELCOME = 001,
    RPL_YOURHOST = 002,
    RPL_CREATED = 003,
    RPL_MYINFO = 004,
    RPL_ISUPPORT = 005,
    RPL_BOUNCE = 010,
    RPL_STATSCOMMANDS = 212,
    RPL_ENDOFSTATS = 219,
    RPL_UMODEIS = 221,
    RPL_STATSUPTIME = 242,
    RPL_LUSERCLIENT = 251,
    RPL_LUSEROP = 252,
    RPL_LUSERUNKNOWN = 253,
    RPL_LUSERCHANNELS = 254,
    RPL_LUSERME = 255,
    RPL_ADMINME = 256,
    RPL_ADMINLOC1 = 257,
    RPL_ADMINLOC2 = 258,
    RPL_ADMINEMAIL = 259,
    RPL_TRYAGAIN = 263,
    RPL_LOCALUSERS = 265,
    RPL_GLOBALUSERS = 266,
    RPL_WHOISCERTFP = 276,
    RPL_NONE = 300,
    RPL_AWAY = 301,
    RPL_USERHOST = 302,
    RPL_UNAWAY = 305,
    RPL_NOWAWAY = 306,
    RPL_WHOISREGNICK = 307,
    RPL_WHOISUSER = 311,
    RPL_WHOISSERVER = 312,
    RPL_WHOISOPERATOR = 313,
    RPL_WHOWASUSER = 314,
    RPL_ENDOFWHO = 315,
    RPL_WHOISIDLE = 317,
    RPL_ENDOFWHOIS = 318,
    RPL_WHOISCHANNELS = 319,
    RPL_WHOISSPECIAL = 320,
    RPL_LISTSTART = 321,
    RPL_LIST = 322,
    RPL_LISTEND = 323,
    RPL_CHANNELMODEIS = 324,
    RPL_CREATIONTIME = 329,
    RPL_WHOISACCOUNT = 330,
    RPL_NOTOPIC = 331,
    RPL_TOPIC = 332,
    RPL_TOPICWHOTIME = 333,
    RPL_INVITELIST = 336,
    RPL_ENDOFINVITELIST = 337,
    RPL_WHOISACTUALLY = 338,
    RPL_INVITING = 341,
    RPL_INVEXLIST = 346,
    RPL_ENDOFINVEXLIST = 347,
    RPL_EXCEPTLIST = 348,
    RPL_ENDOFEXCEPTLIST = 349,
    RPL_VERSION = 351,
    RPL_WHOREPLY = 352,
    RPL_NAMREPLY = 353,
    RPL_LINKS = 364,
    RPL_ENDOFLINKS = 365,
    RPL_ENDOFNAMES = 366,
    RPL_BANLIST = 367,
    RPL_ENDOFBANLIST = 368,
    RPL_ENDOFWHOWAS = 369,
    RPL_INFO = 371,
    RPL_MOTD = 372,
    RPL_ENDOFINFO = 374,
    RPL_MOTDSTART = 375,
    RPL_ENDOFMOTD = 376,
    RPL_WHOISHOST = 378,
    RPL_WHOISMODES = 379,
    RPL_YOUREOPER = 381,
    RPL_REHASHING = 382,
    RPL_TIME = 391,
    ERR_UNKNOWNERROR = 400,
    ERR_NOSUCHNICK = 401,
    ERR_NOSUCHSERVER = 402,
    ERR_NOSUCHCHANNEL = 403,
    ERR_CANNOTSENDTOCHAN = 404,
    ERR_TOOMANYCHANNELS = 405,
    ERR_WASNOSUCHNICK = 406,
    ERR_NOORIGIN = 409,
    ERR_NORECIPIENT = 411,
    ERR_NOTEXTTOSEND = 412,
    ERR_INPUTTOOLONG = 417,
    ERR_UNKNOWNCOMMAND = 421,
    ERR_NOMOTD = 422,
    ERR_NONICKNAMEGIVEN = 431,
    ERR_ERRONEUSNICKNAME = 432,
    ERR_NICKNAMEINUSE = 433,
    ERR_NICKCOLLISION = 436,
    ERR_USERNOTINCHANNEL = 441,
    ERR_NOTONCHANNEL = 442,
    ERR_USERONCHANNEL = 443,
    ERR_NOTREGISTERED = 451,
    ERR_NEEDMOREPARAMS = 461,
    ERR_ALREADYREGISTERED = 462,
    ERR_PASSWDMISMATCH = 464,
    ERR_YOUREBANNEDCREEP = 465,
    ERR_CHANNELISFULL = 471,
    ERR_UNKNOWNMODE = 472,
    ERR_INVITEONLYCHAN = 473,
    ERR_BANNEDFROMCHAN = 474,
    ERR_BADCHANNELKEY = 475,
    ERR_BADCHANMASK = 476,
    ERR_NOPRIVILEGES = 481,
    ERR_CHANOPRIVSNEEDED = 482,
    ERR_CANTKILLSERVER = 483,
    ERR_NOOPERHOST = 491,
    ERR_UMODEUNKNOWNFLAG = 501,
    ERR_USERSDONTMATCH = 502,
    ERR_HELPNOTFOUND = 524,
    ERR_INVALIDKEY = 525,
    RPL_STARTTLS = 670,
    RPL_WHOISSECURE = 671,
    ERR_STARTTLS = 691,
    ERR_INVALIDMODEPARAM = 696,
    RPL_HELPSTART = 704,
    RPL_HELPTXT = 705,
    RPL_ENDOFHELP = 706,
    ERR_NOPRIVS = 723,
    RPL_LOGGEDIN = 900,
    RPL_LOGGEDOUT = 901,
    ERR_NICKLOCKED = 902,
    RPL_SASLSUCCESS = 903,
    ERR_SASLFAIL = 904,
    ERR_SASLTOOLONG = 905,
    ERR_SASLABORTED = 906,
    ERR_SASLALREADY = 907,
    RPL_SASLMECHS = 908,
}

impl From<ResponseCode> for u16 {
    fn from(code: ResponseCode) -> u16 {
        code as u16
    }
}

#[derive(Default)]
pub struct ResponseParams {
    client: String,
    stub: String,
    channel: Option<String>,
    nick: Option<String>,
    host: Option<String>,
    message: Option<String>,
    server: Option<String>,
    modes: Option<String>,
    count: Option<u32>,
    date: Option<String>,
}

impl ResponseParams {
    pub fn new(client: impl Into<String>) -> Self {
        Self {
            client: client.into(),
            stub: "STUBBED_VALUE".to_string(),
            ..Default::default()
        }
    }

    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }
    pub fn nick(mut self, nick: impl Into<String>) -> Self {
        self.nick = Some(nick.into());
        self
    }
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }
    pub fn modes(mut self, modes: impl Into<String>) -> Self {
        self.modes = Some(modes.into());
        self
    }
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

pub trait FormatResponse {
    fn format(&self, params: &ResponseParams) -> String;
}

impl ResponseCode {
    pub fn message(&self, params: ResponseParams) -> String {
        match self {
            // Welcome/Connection Registration (001-015)
            ResponseCode::RPL_WELCOME => format!(
                ":server {} {} :Welcome to the {} Network, {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.client
            ), //"<client> :Welcome to the <networkname> Network, <nick>[!<user>@<host>]"
            ResponseCode::RPL_YOURHOST => format!(
                ":server {} {} :Your host is {}, running version {}\r\n",
                u16::from(*self),
                params.client,
                "OxideIRC\r\n",
                "0.1"
            ), //"<client> :Your host is <servername>, running version <version>"
            ResponseCode::RPL_CREATED => format!(
                ":server {} {} :This server was created {}\r\n",
                u16::from(*self),
                params.client,
                params.date.unwrap_or_default()
            ), //"<client> :This server was created <datetime>"
            ResponseCode::RPL_MYINFO => format!(
                ":server {} {} {} {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <servername> <version> <available user modes> <available channel modes> [<channel modes with a parameter>]"
            ResponseCode::RPL_ISUPPORT => format!(
                ":server {} {} {}, :are supported by this server\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <1-13 tokens> :are supported by this server"
            ResponseCode::RPL_BOUNCE => format!(
                ":server {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <hostname> <port> :<info>"

            // Server Status/Statistics (200-299)
            ResponseCode::RPL_STATSCOMMANDS => format!(
                ":server {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.count.unwrap_or_default()
            ), //"<client> <command> <count>"
            ResponseCode::RPL_ENDOFSTATS => format!(
                ":server {} {} {} :End of STATS report\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <command> :End of STATS report"
            ResponseCode::RPL_STATSUPTIME => format!(
                ":server {} {} :Server Up {}\r\n",
                u16::from(*self),
                params.client,
                params.date.unwrap_or_default()
            ), //"<client> :Server Up <days> days <hours>:<minutes>:<seconds>"
            ResponseCode::RPL_UMODEIS => {
                format!(":server {} {} {}\r\n", u16::from(*self), params.client, "")
            } //"<client> <usermodes>"
            ResponseCode::RPL_LUSERCLIENT => format!(
                ":server {} {} :There are {} users and {} invisible on {} servers\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> :There are <usercount> users and <i> invisible on <servercount> servers"
            ResponseCode::RPL_LUSEROP => format!(
                ":server {} {} {} :operator(s) online\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <operatorcount> :operator(s) online"
            ResponseCode::RPL_LUSERUNKNOWN => format!(
                ":server {} {} {} :unknown connection(s)\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <connections> :unknown connection(s)"
            ResponseCode::RPL_LUSERCHANNELS => format!(
                ":server {} {} {} :channels formed\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <channels> :channels formed"
            ResponseCode::RPL_LUSERME => format!(
                ":server {} {} :I have {} clients and {} servers\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> :I have <clients> clients and <servers> servers"
            ResponseCode::RPL_LOCALUSERS => format!(
                ":server {} {} {} {} :Current local users {}, max {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> [<u> <m>] :Current local users <u>, max <m>"
            ResponseCode::RPL_GLOBALUSERS => format!(
                ":server {} {} {} {} :Current global users {}, max {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> [<u> <m>] :Current global users <u>, max <m>"

            // WHOIS/WHOWAS Responses (300-399)
            ResponseCode::RPL_WHOISUSER => format!(
                ":server {} {} {} {} {} * :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <nick> <username> <host> * :<realname>"
            ResponseCode::RPL_WHOISSERVER => format!(
                ":server {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <nick> <server> :<server info>"
            ResponseCode::RPL_WHOISOPERATOR => format!(
                ":server {} {} {} :is an IRC operator\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <nick> :is an IRC operator"
            ResponseCode::RPL_WHOWASUSER => format!(
                ":server {} {} {} {} {} * :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <nick> <username> <host> * :<realname>"
            ResponseCode::RPL_WHOISIDLE => format!(
                ":server {} {} {} {} :seconds idle since {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <nick> <seconds> <signon>"
            ResponseCode::RPL_ENDOFWHOIS => format!(
                ":server {} {} {} :End of /WHOIS list\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <nick> :End of /WHOIS list"
            ResponseCode::RPL_WHOISCHANNELS => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> :<channels>"
            ResponseCode::RPL_WHOISREGNICK => format!(
                ":server {} {} {} :has identified for this nick\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <nick> :has identified for this nick"
            ResponseCode::RPL_WHOISACCOUNT => format!(
                ":server {} {} {} {} :is logged in as\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> <account> :is logged in as"
            ResponseCode::RPL_WHOISACTUALLY => format!(
                ":server {} {} {} :is actually using host {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> <host|ip> :Is actually using host"
            ResponseCode::RPL_WHOISHOST => format!(
                ":server {} {} {} :is connecting from *@{} {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <nick> :is connecting from *@<host> <ip>"
            ResponseCode::RPL_WHOISMODES => format!(
                ":server {} {} {} :is using modes {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> :is using modes <modes>"
            ResponseCode::RPL_WHOISCERTFP => format!(
                ":server {} {} {} :has client certificate fingerprint {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> :has client certificate fingerprint <fingerprint>"
            ResponseCode::RPL_WHOISSECURE => format!(
                ":server {} {} {} :is using a secure connection\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <nick> :is using a secure connection"
            ResponseCode::RPL_ENDOFWHOWAS => format!(
                ":server {} {} {} :End of WHOWAS\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <nick> :End of WHOWAS"
            ResponseCode::RPL_ENDOFWHO => format!(
                ":server {} {} {} :End of WHO list\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <name> :End of WHO list"
            ResponseCode::RPL_WHOISSPECIAL => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> :blah blah blah"

            // WHO Responses
            ResponseCode::RPL_WHOREPLY => format!(
                ":server {} {} {} {} {} {} {} {} :{} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub,
                params.stub,
                params.stub,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <channel> <user> <host> <server> <nick> <flags> :<hopcount> <realname>"

            // Channel Operations (320-399)
            ResponseCode::RPL_LISTSTART => format!(
                ":server {} {} Channel :Users Name\r\n",
                u16::from(*self),
                params.client
            ), //"<client> Channel :Users Name"
            ResponseCode::RPL_LIST => format!(
                ":server {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub,
                params.stub
            ), //"<client> <channel> <visible> :<topic>"
            ResponseCode::RPL_LISTEND => format!(
                ":server {} {} :End of /LIST\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :End of /LIST"
            ResponseCode::RPL_CHANNELMODEIS => format!(
                ":server {} {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub,
                params.stub
            ), //"<client> <channel> <modestring> <mode arguments>..."
            ResponseCode::RPL_NOTOPIC => format!(
                ":server {} {} {} :No topic is set\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :No topic is set"
            ResponseCode::RPL_TOPIC => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub
            ), //"<client> <channel> :<topic>"
            ResponseCode::RPL_TOPICWHOTIME => format!(
                ":server {} {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub,
                params.stub
            ), //"<client> <channel> <who> <setat>"
            ResponseCode::RPL_NAMREPLY => format!(
                ":server {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                "=",
                params.channel.unwrap_or_default(),
                params.message.unwrap_or_default()
            ), //"<client> <symbol> <channel> :[prefix]<nick>{ [prefix]<nick>}"
            ResponseCode::RPL_ENDOFNAMES => format!(
                ":server {} {} {} :End of /NAMES list\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :End of /NAMES list"
            ResponseCode::RPL_CREATIONTIME => format!(
                ":server {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub
            ), //"<client> <channel> <creationtime>"

            // Channel List Management
            ResponseCode::RPL_INVITELIST => format!(
                ":server {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel>"
            ResponseCode::RPL_ENDOFINVITELIST => format!(
                ":server {} {} :End of /INVITE list\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :End of /INVITE list"
            ResponseCode::RPL_INVITING => format!(
                ":server {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.channel.unwrap_or_default()
            ), //"<client> <nick> <channel>"
            ResponseCode::RPL_INVEXLIST => format!(
                ":server {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub
            ), //"<client> <channel> <mask>"
            ResponseCode::RPL_ENDOFINVEXLIST => format!(
                ":server {} {} {} :End of Channel Invite Exception List\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :End of Channel Invite Exception List"
            ResponseCode::RPL_EXCEPTLIST => format!(
                ":server {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub
            ), //"<client> <channel> <mask>"
            ResponseCode::RPL_ENDOFEXCEPTLIST => format!(
                ":server {} {} {} :End of channel exception list\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :End of channel exception list"
            ResponseCode::RPL_BANLIST => format!(
                ":server {} {} {} {} {} {}\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default(),
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <channel> <mask> <who> <set-ts>"
            ResponseCode::RPL_ENDOFBANLIST => format!(
                ":server {} {} {} :End of channel ban list\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :End of channel ban list"

            // Server Information (370-399)
            ResponseCode::RPL_MOTD => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :<line of motd>"
            ResponseCode::RPL_MOTDSTART => format!(
                ":server {} {} :- {} Message of the day - \r\n",
                u16::from(*self),
                params.client,
                params.server.unwrap_or_default()
            ), //"<client> :- <server> Message of the day - "
            ResponseCode::RPL_ENDOFMOTD => format!(
                ":server {} {} :End of /MOTD command.\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :End of /MOTD command"
            ResponseCode::RPL_VERSION => format!(
                ":server {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <version> <server> :<comments>"
            ResponseCode::RPL_TIME => format!(
                ":server {} {} {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> <server> <timestamp> <offset> :<human-readable time>"
            ResponseCode::RPL_INFO => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :<string>"
            ResponseCode::RPL_ENDOFINFO => format!(
                ":server {} {} :End of INFO list\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :End of INFO list"
            ResponseCode::RPL_LINKS => format!(
                ":server {} {} * {} :{} {}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub,
                params.stub
            ), //"<client> * <server> :<hopcount> <server info>"
            ResponseCode::RPL_ENDOFLINKS => format!(
                ":server {} {} * :End of /LINKS list\r\n",
                u16::from(*self),
                params.client
            ), //"<client> * :End of /LINKS list"

            // User Status
            ResponseCode::RPL_AWAY => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <nick> :<message>"
            ResponseCode::RPL_USERHOST => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :[<reply>{ <reply>}]"
            ResponseCode::RPL_UNAWAY => format!(
                ":server {} {} :You are no longer marked as being away\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You are no longer marked as being away"
            ResponseCode::RPL_NOWAWAY => format!(
                ":server {} {} :You have been marked as being away\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You have been marked as being away"

            // Administrative Information
            ResponseCode::RPL_ADMINME => format!(
                ":server {} {} :Administrative info\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Administrative info"
            ResponseCode::RPL_ADMINLOC1 => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :<info>"
            ResponseCode::RPL_ADMINLOC2 => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :<info>"
            ResponseCode::RPL_ADMINEMAIL => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :<info>"

            // Operator Commands
            ResponseCode::RPL_YOUREOPER => format!(
                ":server {} {} :You are now an IRC operator\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You are now an IRC operator"
            ResponseCode::RPL_REHASHING => format!(
                ":server {} {} {} :Rehashing\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <config file> :Rehashing"

            // Error Responses (400-599)
            ResponseCode::ERR_NOSUCHNICK => format!(
                ":server {} {} {} :No such nick/channel\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nickname> :No such nick/channel"
            ResponseCode::ERR_NOSUCHSERVER => format!(
                ":server {} {} {} :No such server\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <server name> :No such server"
            ResponseCode::ERR_NOSUCHCHANNEL => format!(
                ":server {} {} {} :No such channel\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :No such channel"
            ResponseCode::ERR_CANNOTSENDTOCHAN => format!(
                ":server {} {} {} :Cannot send to channel\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Cannot send to channel"
            ResponseCode::ERR_TOOMANYCHANNELS => format!(
                ":server {} {} {} :You have joined too many channels\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :You have joined too many channels"
            ResponseCode::ERR_WASNOSUCHNICK => format!(
                ":server {} {} {} :There was no such nickname\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nickname> :There was no such nickname"
            ResponseCode::ERR_NOORIGIN => format!(
                ":server {} {} :No origin specified\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :No origin specified"
            ResponseCode::ERR_NORECIPIENT => format!(
                ":server {} {} :No recipient given ({})\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :No recipient given (<command>)"
            ResponseCode::ERR_NOTEXTTOSEND => format!(
                ":server {} {} :No text to send\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :No text to send"
            ResponseCode::ERR_INPUTTOOLONG => format!(
                ":server {} {} :Input line was too long\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Input line was too long"
            ResponseCode::ERR_UNKNOWNCOMMAND => format!(
                ":server {} {} {} :Unknown command\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <command> :Unknown command"
            ResponseCode::ERR_NOMOTD => format!(
                ":server {} {} :MOTD File is missing\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :MOTD File is missing"
            ResponseCode::ERR_NONICKNAMEGIVEN => format!(
                ":server {} {} :No nickname given\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :No nickname given"
            ResponseCode::ERR_ERRONEUSNICKNAME => format!(
                ":server {} {} {} :Erroneous nickname\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nick> :Erroneous nickname"
            ResponseCode::ERR_NICKNAMEINUSE => format!(
                ":server {} {} {} :Nickname is already in use\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nick> :Nickname is already in use"
            ResponseCode::ERR_NICKCOLLISION => format!(
                ":server {} {} {} :Nickname collision KILL\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nick> :Nickname collision KILL"
            ResponseCode::ERR_USERNOTINCHANNEL => format!(
                ":server {} {} {} {} :They aren't on that channel\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default(),
                params.channel.unwrap_or_default()
            ), //"<client> <nick> <channel> :They aren't on that channel"
            ResponseCode::ERR_NOTONCHANNEL => format!(
                ":server {} {} {} :You're not on that channel\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :You're not on that channel"
            ResponseCode::ERR_USERONCHANNEL => format!(
                ":server {} {} {} {} :is already on channel\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.channel.unwrap_or_default()
            ), //"<client> <user> <channel> :is already on channel"
            ResponseCode::ERR_CHANNELISFULL => format!(
                ":server {} {} {} :Cannot join channel (+l)\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Cannot join channel (+l)"
            ResponseCode::ERR_UNKNOWNMODE => format!(
                ":server {} {} {} :is unknown mode char to me\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <char> :is unknown mode char to me"
            ResponseCode::ERR_INVITEONLYCHAN => format!(
                ":server {} {} {} :Cannot join channel (+i)\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Cannot join channel (+i)"
            ResponseCode::ERR_BANNEDFROMCHAN => format!(
                ":server {} {} {} :Cannot join channel (+b)\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Cannot join channel (+b)"
            ResponseCode::ERR_BADCHANNELKEY => format!(
                ":server {} {} {} :Cannot join channel (+k)\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Cannot join channel (+k)"
            ResponseCode::ERR_BADCHANMASK => format!(
                ":server {} {} {} :Bad Channel Mask\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :Bad Channel Mask"
            ResponseCode::ERR_NOTREGISTERED => format!(
                ":server {} {} :You have not registered\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You have not registered"
            ResponseCode::ERR_NEEDMOREPARAMS => format!(
                ":server {} {} {} :Not enough parameters\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <command> :Not enough parameters"
            ResponseCode::ERR_ALREADYREGISTERED => format!(
                ":server {} {} :You may not reregister\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You may not reregister"
            ResponseCode::ERR_PASSWDMISMATCH => format!(
                ":server {} {} :Password incorrect\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Password incorrect"
            ResponseCode::ERR_YOUREBANNEDCREEP => format!(
                ":server {} {} :You are banned from this server\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You are banned from this server"
            ResponseCode::ERR_NOPRIVILEGES => format!(
                ":server {} {} :Permission Denied- You're not an IRC operator\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Permission Denied- You're not an IRC operator"
            ResponseCode::ERR_CHANOPRIVSNEEDED => format!(
                ":server {} {} {} :You're not channel operator\r\n",
                u16::from(*self),
                params.client,
                params.channel.unwrap_or_default()
            ), //"<client> <channel> :You're not channel operator"
            ResponseCode::ERR_CANTKILLSERVER => format!(
                ":server {} {} :You cant kill a server!\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You cant kill a server!"
            ResponseCode::ERR_NOOPERHOST => format!(
                ":server {} {} :No O-lines for your host\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :No O-lines for your host"
            ResponseCode::ERR_UMODEUNKNOWNFLAG => format!(
                ":server {} {} :Unknown MODE flag\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Unknown MODE flag"
            ResponseCode::ERR_USERSDONTMATCH => format!(
                ":server {} {} :Cant change mode for other users\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Cant change mode for other users"
            ResponseCode::ERR_HELPNOTFOUND => format!(
                ":server {} {} {} :No help available on this topic\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <subject> :No help available on this topic"
            ResponseCode::ERR_INVALIDKEY => format!(
                ":server {} {} :Key is not valid for this server\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :Key is not valid for this server"
            ResponseCode::ERR_UNKNOWNERROR => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <command> :<info>"
            ResponseCode::ERR_INVALIDMODEPARAM => format!(
                ":server {} {} {} {} :Invalid mode parameter\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.channel.unwrap_or_default()
            ), //"<client> <mode> <channel> :Invalid mode parameter"
            ResponseCode::ERR_NOPRIVS => format!(
                ":server {} {} {} :Insufficient oper privileges.\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <priv> :Insufficient oper privileges."
            ResponseCode::RPL_STARTTLS => format!(
                ":server {} {} :STARTTLS successful, proceed with TLS handshake\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :STARTTLS successful, proceed with TLS handshake"
            ResponseCode::ERR_STARTTLS => format!(
                ":server {} {} :STARTTLS failed ({})\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> :STARTTLS failed (<reason>)"
            ResponseCode::RPL_TRYAGAIN => format!(
                ":server {} {} {} :Please wait a while and try again.\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <command> :Please wait a while and try again."

            // SASL Authentication (900-999)
            ResponseCode::RPL_LOGGEDIN => format!(
                ":server {} {} {} {} :You are now logged in as {}\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default(),
                params.stub,
                params.stub
            ), //"<client> <nick> <user> <account> :You are now logged in as <account>"
            ResponseCode::RPL_LOGGEDOUT => format!(
                ":server {}{} {} :You are now logged out\r\n",
                u16::from(*self),
                params.client,
                params.nick.unwrap_or_default()
            ), //"<client> <nick> :You are now logged out"
            ResponseCode::RPL_SASLSUCCESS => format!(
                ":server {} {} :SASL authentication successful\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :SASL authentication successful"
            ResponseCode::RPL_SASLMECHS => format!(
                ":server {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub
            ), //"<client> <mechanisms> :are available SASL mechanisms"
            ResponseCode::ERR_NICKLOCKED => format!(
                ":server {} {} :You must use a nick assigned to you\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You must use a nick assigned to you"
            ResponseCode::ERR_SASLFAIL => format!(
                ":server {} {} :SASL authentication failed\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :SASL authentication failed"
            ResponseCode::ERR_SASLTOOLONG => format!(
                ":server {} {} :SASL message too long\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :SASL message too long"
            ResponseCode::ERR_SASLABORTED => format!(
                ":server {} {} :SASL authentication aborted\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :SASL authentication aborted"
            ResponseCode::ERR_SASLALREADY => format!(
                ":server {} {} :You have already authenticated using SASL\r\n",
                u16::from(*self),
                params.client
            ), //"<client> :You have already authenticated using SASL"

            // Help System
            ResponseCode::RPL_HELPSTART => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <subject> :<first line of help section>"
            ResponseCode::RPL_HELPTXT => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <subject> :<line of help text>"
            ResponseCode::RPL_ENDOFHELP => format!(
                ":server {} {} {} :{}\r\n",
                u16::from(*self),
                params.client,
                params.stub,
                params.stub
            ), //"<client> <subject> :<last line of help text>"

            // Special Cases
            ResponseCode::RPL_NONE => format!(":server {}\r\n", u16::from(*self)), //"Undefined format"
        }
    }
}
