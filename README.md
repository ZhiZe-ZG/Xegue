# Xegue

A simple game inspired by rogue.

Reference:

- [Rogue 5.4.4](http://rogue.rogueforge.net/rogue-5-4/)

ToDo:

```text
armor.c
chase.c
command.c
config.guess
config.h.in
config.sub
configure
configure.ac
daemon.c
daemons.c
extern.c
extern.h
fight.c
init.c
install-sh
io.c
✅ LICENSE.TXT
list.c
mach_dep.c
main.c
Makefile.in
Makefile.std
mdport.c
misc.c
monsters.c
move.c
new_level.c
options.c
pack.c
passages.c
potions.c
rings.c
rip.c
rogue.6.in
rogue.cat.in
rogue.desktop
rogue.doc.in
✅ rogue.h data structures, enumerations and function list
rogue.html.in
rogue.me.in
rogue.png
rogue.spec
rogue54.sln
rogue54.vcproj
rooms.c
save.c
score.h
scrolls.c
state.c
sticks.c
things.c
vers.c
weapons.c
wizard.c
xcrypt.c
```

`rogue.h`

```c
/*
 * Rogue definitions and variable declarations
 *
 * @(#)rogue.h	5.42 (Berkeley) 08/06/83
 *
 * Rogue: Exploring the Dungeons of Doom
 * Copyright (C) 1980-1983, 1985, 1999 Michael Toy, Ken Arnold and Glenn Wichman
 * All rights reserved.
 *
 * See the file LICENSE.TXT for full copyright and licensing information.
 */

#include "extern.h"

#undef lines 

#define NOOP(x) (x += 0)
#define CCHAR(x) ( (char) (x & A_CHARTEXT) )
/*
 * Maximum number of different things
 */
#define MAXROOMS	9
#define MAXTHINGS	9
#define MAXOBJ		9
#define MAXPACK		23
#define MAXTRAPS	10
#define AMULETLEVEL	26
#define	NUMTHINGS	7	/* number of types of things */
#define MAXPASS		13	/* upper limit on number of passages */
#define	NUMLINES	24
#define	NUMCOLS		80
#define STATLINE		(NUMLINES - 1)
#define BORE_LEVEL	50

/*
 * return values for get functions
 */
#define	NORM	0	/* normal exit */
#define	QUIT	1	/* quit option setting */
#define	MINUS	2	/* back up one option */

/*
 * inventory types
 */
#define	INV_OVER	0
#define	INV_SLOW	1
#define	INV_CLEAR	2

/*
 * All the fun defines
 */
#define when		break;case
#define otherwise	break;default
#define until(expr)	while(!(expr))
#define next(ptr)	(*ptr).l_next
#define prev(ptr)	(*ptr).l_prev
#define winat(y,x)	(moat(y,x) != NULL ? moat(y,x)->t_disguise : chat(y,x))
#define ce(a,b)		((a).x == (b).x && (a).y == (b).y)
#define hero		player.t_pos
#define pstats		player.t_stats
#define pack		player.t_pack
#define proom		player.t_room
#define max_hp		player.t_stats.s_maxhp
#define attach(a,b)	_attach(&a,b)
#define detach(a,b)	_detach(&a,b)
#define free_list(a)	_free_list(&a)
#undef max
#define max(a,b)	((a) > (b) ? (a) : (b))
#define on(thing,flag)	((bool)(((thing).t_flags & (flag)) != 0))
#define GOLDCALC	(rnd(50 + 10 * level) + 2)
#define ISRING(h,r)	(cur_ring[h] != NULL && cur_ring[h]->o_which == r)
#define ISWEARING(r)	(ISRING(LEFT, r) || ISRING(RIGHT, r))
#define ISMULT(type) 	(type == POTION || type == SCROLL || type == FOOD)
#define INDEX(y,x)	(&places[((x) << 5) + (y)])
#define chat(y,x)	(places[((x) << 5) + (y)].p_ch)
#define flat(y,x)	(places[((x) << 5) + (y)].p_flags)
#define moat(y,x)	(places[((x) << 5) + (y)].p_monst)
#define unc(cp)		(cp).y, (cp).x
#ifdef MASTER
#define debug		if (wizard) msg
#endif

/*
 ✅  * things that appear on the screens
 */
#define PASSAGE		'#'
#define DOOR		'+'
#define FLOOR		'.'
#define PLAYER		'@'
#define TRAP		'^'
#define STAIRS		'%'
#define GOLD		'*'
#define POTION		'!'
#define SCROLL		'?'
#define MAGIC		'$'
#define FOOD		':'
#define WEAPON		')'
#define ARMOR		']'
#define AMULET		','
#define RING		'='
#define STICK		'/'
#define CALLABLE	-1 // not used
#define R_OR_S		-2 // not used

/*
 * Various constants
 */
#define BEARTIME	spread(3)
#define SLEEPTIME	spread(5)
#define HOLDTIME	spread(2)
#define WANDERTIME	spread(70)
#define BEFORE		spread(1)
#define AFTER		spread(2)
#define HEALTIME	30
#define HUHDURATION	20
#define SEEDURATION	850
#define HUNGERTIME	1300
#define MORETIME	150
#define STOMACHSIZE	2000
#define STARVETIME	850
#define ESCAPE		27
#define LEFT		0
#define RIGHT		1
#define BOLT_LENGTH	6
#define LAMPDIST	3
#ifdef MASTER
#ifndef PASSWD
#define	PASSWD		"mTBellIQOsLNA"
#endif
#endif

/*
 * Save against things
 */
#define VS_POISON	00
#define VS_PARALYZATION	00
#define VS_DEATH	00
#define VS_BREATH	02
#define VS_MAGIC	03

/*
 * Various flag bits
 */
/* flags for rooms */
#define ISDARK	0000001		/* room is dark */
#define ISGONE	0000002		/* room is gone (a corridor) */
#define ISMAZE	0000004		/* room is gone (a corridor) */

/* flags for objects */
#define ISCURSED 000001		/* object is cursed */
#define ISKNOW	0000002		/* player knows details about the object */
#define ISMISL	0000004		/* object is a missile type */
#define ISMANY	0000010		/* object comes in groups */
/*	ISFOUND 0000020		...is used for both objects and creatures */
#define	ISPROT	0000040		/* armor is permanently protected */

/* flags for creatures */
#define CANHUH	0000001		/* creature can confuse */
#define CANSEE	0000002		/* creature can see invisible creatures */
#define ISBLIND	0000004		/* creature is blind */
#define ISCANC	0000010		/* creature has special qualities cancelled */
#define ISLEVIT	0000010		/* hero is levitating */
#define ISFOUND	0000020		/* creature has been seen (used for objects) */
#define ISGREED	0000040		/* creature runs to protect gold */
#define ISHASTE	0000100		/* creature has been hastened */
#define ISTARGET 000200		/* creature is the target of an 'f' command */
#define ISHELD	0000400		/* creature has been held */
#define ISHUH	0001000		/* creature is confused */
#define ISINVIS	0002000		/* creature is invisible */
#define ISMEAN	0004000		/* creature can wake when player enters room */
#define ISHALU	0004000		/* hero is on acid trip */
#define ISREGEN	0010000		/* creature can regenerate */
#define ISRUN	0020000		/* creature is running at the player */
#define SEEMONST 040000		/* hero can detect unseen monsters */
#define ISFLY	0040000		/* creature can fly */
#define ISSLOW	0100000		/* creature has been slowed */

/*
 * Flags for level map
 */
#define F_PASS		0x80		/* is a passageway */
#define F_SEEN		0x40		/* have seen this spot before */
#define F_DROPPED	0x20		/* object was dropped here */
#define F_LOCKED	0x20		/* door is locked */
#define F_REAL		0x10		/* what you see is what you get */
#define F_PNUM		0x0f		/* passage number mask */
#define F_TMASK		0x07		/* trap number mask */

/*
 * Trap types
 */
#define T_DOOR	00
#define T_ARROW	01
#define T_SLEEP	02
#define T_BEAR	03
#define T_TELEP	04
#define T_DART	05
#define T_RUST	06
#define T_MYST  07
#define NTRAPS	8

/*
 * Potion types
 */
#define P_CONFUSE	0
#define P_LSD		1
#define P_POISON	2
#define P_STRENGTH	3
#define P_SEEINVIS	4
#define P_HEALING	5
#define P_MFIND		6
#define	P_TFIND 	7
#define	P_RAISE		8
#define P_XHEAL		9
#define P_HASTE		10
#define P_RESTORE	11
#define P_BLIND		12
#define P_LEVIT		13
#define MAXPOTIONS	14

/*
 * Scroll types
 */
#define S_CONFUSE	0
#define S_MAP		1
#define S_HOLD		2
#define S_SLEEP		3
#define S_ARMOR		4
#define S_ID_POTION	5
#define S_ID_SCROLL	6
#define S_ID_WEAPON	7
#define S_ID_ARMOR	8
#define S_ID_R_OR_S	9
#define S_SCARE		10
#define S_FDET		11
#define S_TELEP		12
#define S_ENCH		13
#define S_CREATE	14
#define S_REMOVE	15
#define S_AGGR		16
#define S_PROTECT	17
#define MAXSCROLLS	18

/*
 * Weapon types
 */
#define MACE		0
#define SWORD		1
#define BOW		2
#define ARROW		3
#define DAGGER		4
#define TWOSWORD	5
#define DART		6
#define SHIRAKEN	7
#define SPEAR		8
#define FLAME		9	/* fake entry for dragon breath (ick) */
#define MAXWEAPONS	9	/* this should equal FLAME */

/*
 * Armor types
 */
#define LEATHER		0
#define RING_MAIL	1
#define STUDDED_LEATHER	2
#define SCALE_MAIL	3
#define CHAIN_MAIL	4
#define SPLINT_MAIL	5
#define BANDED_MAIL	6
#define PLATE_MAIL	7
#define MAXARMORS	8

/*
 * Ring types
 */
#define R_PROTECT	0
#define R_ADDSTR	1
#define R_SUSTSTR	2
#define R_SEARCH	3
#define R_SEEINVIS	4
#define R_NOP		5
#define R_AGGR		6
#define R_ADDHIT	7
#define R_ADDDAM	8
#define R_REGEN		9
#define R_DIGEST	10
#define R_TELEPORT	11
#define R_STEALTH	12
#define R_SUSTARM	13
#define MAXRINGS	14

/*
 * Rod/Wand/Staff types
 */
#define WS_LIGHT	0
#define WS_INVIS	1
#define WS_ELECT	2
#define WS_FIRE		3
#define WS_COLD		4
#define WS_POLYMORPH	5
#define WS_MISSILE	6
#define WS_HASTE_M	7
#define WS_SLOW_M	8
#define WS_DRAIN	9
#define WS_NOP		10
#define WS_TELAWAY	11
#define WS_TELTO	12
#define WS_CANCEL	13
#define MAXSTICKS	14

/*
 * Now we define the structures and types
 */

/*
 * Help list
 */
struct h_list {
    char h_ch;
    char *h_desc;
    bool h_print;
};

/*
 ✅ * Coordinate data type
 */
typedef struct {
    int x;
    int y;
} coord;

typedef unsigned int str_t;

/*
 * Stuff about objects
 */
struct obj_info {
    char *oi_name;
    int oi_prob;
    int oi_worth;
    char *oi_guess;
    bool oi_know;
};

/*
 * Room structure
 */
struct room {
    coord r_pos;			/* Upper left corner */
    coord r_max;			/* Size of room */
    coord r_gold;			/* Where the gold is */
    int r_goldval;			/* How much the gold is worth */
    short r_flags;			/* info about the room */
    int r_nexits;			/* Number of exits */
    coord r_exit[12];			/* Where the exits are */
};

/*
 * Structure describing a fighting being
 */
struct stats {
    str_t s_str;			/* Strength */
    int s_exp;				/* Experience */
    int s_lvl;				/* level of mastery */
    int s_arm;				/* Armor class */
    int s_hpt;			/* Hit points */
    char s_dmg[13];			/* String describing damage done */
    int  s_maxhp;			/* Max hit points */
};

/*
 ✅* Structure for monsters and player
 */
union thing {
    struct {
	union thing *_l_next, *_l_prev;	/* Next pointer in link */
	coord _t_pos;			/* Position */
	bool _t_turn;			/* If slowed, is it a turn to move */
	char _t_type;			/* What it is */
	char _t_disguise;		/* What mimic looks like */
	char _t_oldch;			/* Character that was where it was */
	coord *_t_dest;			/* Where it is running to */
	short _t_flags;			/* State word */
	struct stats _t_stats;		/* Physical description */
	struct room *_t_room;		/* Current room for thing */
	union thing *_t_pack;		/* What the thing is carrying */
        int _t_reserved;
    } _t;
    struct {
	union thing *_l_next, *_l_prev;	/* Next pointer in link */
	int _o_type;			/* What kind of object it is */
	coord _o_pos;			/* Where it lives on the screen */
	char *_o_text;			/* What it says if you read it */
	int  _o_launch;			/* What you need to launch it */
	char _o_packch;			/* What character it is in the pack */
	char _o_damage[8];		/* Damage if used like sword */
	char _o_hurldmg[8];		/* Damage if thrown */
	int _o_count;			/* count for plural objects */
	int _o_which;			/* Which object of a type it is */
	int _o_hplus;			/* Plusses to hit */
	int _o_dplus;			/* Plusses to damage */
	int _o_arm;			/* Armor protection */
	int _o_flags;			/* information about objects */
	int _o_group;			/* group number for this object */
	char *_o_label;			/* Label for object */
    } _o;
};

//✅ typedef union thing THING;

#define l_next		_t._l_next
#define l_prev		_t._l_prev
#define t_pos		_t._t_pos
#define t_turn		_t._t_turn
#define t_type		_t._t_type
#define t_disguise	_t._t_disguise
#define t_oldch		_t._t_oldch
#define t_dest		_t._t_dest
#define t_flags		_t._t_flags
#define t_stats		_t._t_stats
#define t_pack		_t._t_pack
#define t_room		_t._t_room
#define t_reserved      _t._t_reserved
#define o_type		_o._o_type
#define o_pos		_o._o_pos
#define o_text		_o._o_text
#define o_launch	_o._o_launch
#define o_packch	_o._o_packch
#define o_damage	_o._o_damage
#define o_hurldmg	_o._o_hurldmg
#define o_count		_o._o_count
#define o_which		_o._o_which
#define o_hplus		_o._o_hplus
#define o_dplus		_o._o_dplus
#define o_arm		_o._o_arm
#define o_charges	o_arm
#define o_goldval	o_arm
#define o_flags		_o._o_flags
#define o_group		_o._o_group
#define o_label		_o._o_label

/*
✅  * describe a place on the level map
 */
typedef struct {
    char p_ch;
    char p_flags;
    THING *p_monst;
} PLACE;

/*
 * Array containing information on all the various types of monsters
 */
struct monster {
    char *m_name;			/* What to call the monster */
    int m_carry;			/* Probability of carrying something */
    short m_flags;			/* things about the monster */
    struct stats m_stats;		/* Initial stats */
};

/*
 * External variables
 */

extern bool	after, again, allscore, amulet, door_stop, fight_flush,
		firstmove, has_hit, inv_describe, jump, kamikaze,
		lower_msg, move_on, msg_esc, pack_used[],
		passgo, playing, q_comm, running, save_msg, see_floor,
		seenstairs, stat_msg, terse, to_death, tombstone;

extern char	dir_ch, file_name[], home[], huh[], *inv_t_name[],
		l_last_comm, l_last_dir, last_comm, last_dir, *Numname,
		outbuf[], *p_colors[], *r_stones[], *release, runch,
		*s_names[], take, *tr_name[], *ws_made[], *ws_type[];

extern int	a_class[], count, food_left, hungry_state, inpack,
		inv_type, lastscore, level, max_hit, max_level, mpos,
		n_objs, no_command, no_food, no_move, noscore, ntraps, purse,
		quiet, vf_hit;

extern unsigned int	numscores;

extern int	dnum, e_levels[], seed;

extern WINDOW	*hw;

extern coord	delta, oldpos, stairs;

extern PLACE	places[];

extern THING	*cur_armor, *cur_ring[], *cur_weapon, *l_last_pick,
		*last_pick, *lvl_obj, *mlist, player;

extern struct h_list	helpstr[];

extern struct room	*oldrp, passages[], rooms[];

extern struct stats	max_stats;

extern struct monster	monsters[];

extern struct obj_info	arm_info[], pot_info[], ring_info[],
			scr_info[], things[], ws_info[], weap_info[];

/*
 * Function types
 */
void	_attach(THING **list, THING *item);
void	_detach(THING **list, THING *item);
void	_free_list(THING **ptr);
void	addmsg(char *fmt, ...);
bool	add_haste(bool potion);
void	add_pack(THING *obj, bool silent);
void	add_pass();
void	add_str(str_t *sp, int amt);
void	accnt_maze(int y, int x, int ny, int nx);
void	aggravate();
int	attack(THING *mp);
void	badcheck(char *name, struct obj_info *info, int bound);
void	bounce(THING *weap, char *mname, bool noend);
void	call();
void	call_it(struct obj_info *info);
bool	cansee(int y, int x);
int	center(char *str);
void	chg_str(int amt);
void	check_level();
void	conn(int r1, int r2);
void	command();
void	create_obj();

void	current(THING *cur, char *how, char *where);
void	d_level();
void	death(char monst);
char	death_monst();
void	dig(int y, int x);
void	discard(THING *item);
void	discovered();
int	dist(int y1, int x1, int y2, int x2);
int	dist_cp(coord *c1, coord *c2);
int	do_chase(THING *th);
void	do_daemons(int flag);
void	do_fuses(int flag);
void	do_maze(struct room *rp);
void	do_motion(THING *obj, int ydelta, int xdelta);
void	do_move(int dy, int dx);
void	do_passages();
void	do_pot(int type, bool knowit);
void	do_rooms();
void	do_run(char ch);
void	do_zap();
void	doadd(char *fmt, va_list args);
void	door(struct room *rm, coord *cp);
void	door_open(struct room *rp);
void	drain();
//✅ void	draw_room(struct room *rp);
void	drop();
void	eat();
size_t  encread(char *start, size_t size, FILE *inf);
size_t	encwrite(char *start, size_t size, FILE *outf);
int	endmsg();
void	enter_room(coord *cp);
void	erase_lamp(coord *pos, struct room *rp);
int	exp_add(THING *tp);
void	extinguish(void (*func)());
void	fall(THING *obj, bool pr);
void	fire_bolt(coord *start, coord *dir, char *name);
char	floor_at();
void	flush_type();
int	fight(coord *mp, THING *weap, bool thrown);
void	fix_stick(THING *cur);
void	fuse(void (*func)(), int arg, int time, int type);
bool	get_dir();
int	gethand();
void	give_pack(THING *tp);
void	help();
void	hit(char *er, char *ee, bool noend);
//✅ void	horiz(struct room *rp, int starty);
void	leave_room(coord *cp);
void	lengthen(void (*func)(), int xtime);
void	look(bool wakeup);
int	hit_monster(int y, int x, THING *obj);
void	identify();
void	illcom(int ch);
void	init_check();
void	init_colors();
void	init_materials();
void	init_names();
void	init_player();
void	init_probs();
void	init_stones();
void	init_weapon(THING *weap, int which);
bool	inventory(THING *list, int type);
void	invis_on();
void	killed(THING *tp, bool pr);
void	kill_daemon(void (*func)());
bool	lock_sc();
void	miss(char *er, char *ee, bool noend);
void	missile(int ydelta, int xdelta);
void	money(int value);
int	move_monst(THING *tp);
void	move_msg(THING *obj);
int	msg(char *fmt, ...);
void	nameit(THING *obj, char *type, char *which, struct obj_info *op, char *(*prfunc)(THING *));
void	new_level();
void	new_monster(THING *tp, char type, coord *cp);
void	numpass(int y, int x);
void	option();
void	open_score();
void	parse_opts(char *str);
void 	passnum();
char	*pick_color(char *col);
int	pick_one(struct obj_info *info, int nitems);
void	pick_up(char ch);
void	picky_inven();
void	pr_spec(struct obj_info *info, int nitems);
void	pr_list();
void	put_bool(void *b);
void	put_inv_t(void *ip);
void	put_str(void *str);
void	put_things();
void	putpass(coord *cp);
void	quaff();
void	raise_level();
char	randmonster(bool wander);
void	read_scroll();
void    relocate(THING *th, coord *new_loc);
void	remove_mon(coord *mp, THING *tp, bool waskill);
void	reset_last();
bool	restore(char *file, char **envp);
int	ring_eat(int hand);
void	ring_on();
void	ring_off();
int	rnd(int range);
int	rnd_room();
int	roll(int number, int sides);
int	rs_save_file(FILE *savef);
int	rs_restore_file(FILE *inf);
void	runto(coord *runner);
void	rust_armor(THING *arm);
int	save(int which);
void	save_file(FILE *savef);
void	save_game();
int	save_throw(int which, THING *tp);
void	score(int amount, int flags, char monst);
void	search();
void	set_know(THING *obj, struct obj_info *info);
void	set_oldch(THING *tp, coord *cp);
void	setup();
void	shell();
bool	show_floor();
void	show_map();
void	show_win(char *message);
int	sign(int nm);
int	spread(int nm);
void	start_daemon(void (*func)(), int arg, int type);
void	start_score();
void	status();
int	step_ok(int ch);
void	strucpy(char *s1, char *s2, int len);
int	swing(int at_lvl, int op_arm, int wplus);
void	take_off();
void	teleport();
void	total_winner();
void	thunk(THING *weap, char *mname, bool noend);
void	treas_room();
void	turnref();
void	u_level();
void	uncurse(THING *obj);
void	unlock_sc();
//✅ void	vert(struct room *rp, int startx);
void	wait_for(int ch);
THING  *wake_monster(int y, int x);
void	wanderer();
void	waste_time();
void	wear();
void	whatis(bool insist, int type);
void	wield();

bool	chase(THING *tp, coord *ee);
bool	diag_ok(coord *sp, coord *ep);
bool	dropcheck(THING *obj);
bool	fallpos(coord *pos, coord *newpos);
bool	find_floor(struct room *rp, coord *cp, int limit, bool monst);
bool	is_magic(THING *obj);
bool    is_symlink(char *sp); 
bool	levit_check();
bool	pack_room(bool from_floor, THING *obj);
bool	roll_em(THING *thatt, THING *thdef, THING *weap, bool hurl);
bool	see_monst(THING *mp);
bool	seen_stairs();
bool	turn_ok(int y, int x);
bool	turn_see(bool turn_off);
bool	is_current(THING *obj);
int	passwd();

char	be_trapped(coord *tc);
char	floor_ch();
char	pack_char();
char	readchar();
char	rnd_thing();

char	*charge_str(THING *obj);
char	*choose_str(char *ts, char *ns);
char	*inv_name(THING *obj, bool drop);
char	*nullstr(THING *ignored);
char	*num(int n1, int n2, char type);
char	*ring_num(THING *obj);
char	*set_mname(THING *tp);
char	*vowelstr(char *str);

int	get_bool(void *vp, WINDOW *win);
int	get_inv_t(void *vp, WINDOW *win);
int	get_num(void *vp, WINDOW *win);
int	get_sf(void *vp, WINDOW *win);
int	get_str(void *vopt, WINDOW *win);
int	trip_ch(int y, int x, int ch);

coord	*find_dest(THING *tp);
coord	*rndmove(THING *who);

THING	*find_obj(int y, int x);
THING	*get_item(char *purpose, int type);
THING	*leave_pack(THING *obj, bool newobj, bool all);
THING	*new_item();
THING	*new_thing();

struct room	*roomin(coord *cp);

#define MAXDAEMONS 20

extern struct delayed_action {
    int d_type;
    void (*d_func)();
    int d_arg;
    int d_time;
} d_list[MAXDAEMONS];

typedef struct {
    char	*st_name;
    int		st_value;
} STONE;

extern int      total;
extern int      between;
extern int      group;
extern coord    nh;
extern char     *rainbow[];
extern int      cNCOLORS;
extern STONE    stones[];
extern int      cNSTONES;
extern char     *wood[];
extern int      cNWOOD;
extern char     *metal[];
extern int      cNMETAL;
```

extern.h

```c
/*
 * Defines for things used in mach_dep.c
 *
 * @(#)extern.h	4.35 (Berkeley) 02/05/99
 *
 * Rogue: Exploring the Dungeons of Doom
 * Copyright (C) 1980-1983, 1985, 1999 Michael Toy, Ken Arnold and Glenn Wichman
 * All rights reserved.
 *
 * See the file LICENSE.TXT for full copyright and licensing information.
 */


#ifdef HAVE_CONFIG_H
#ifdef PDCURSES
#undef HAVE_UNISTD_H
#undef HAVE_LIMITS_H
#undef HAVE_MEMORY_H
#undef HAVE_STRING_H
#endif
#include "config.h"
#elif defined(__DJGPP__)
#define HAVE_SYS_TYPES_H 1
#define HAVE_PROCESS_H 1
#define HAVE_PWD_H 1
#define HAVE_TERMIOS_H 1
#define HAVE_SETGID 1
#define HAVE_GETGID 1
#define HAVE_SETUID 1
#define HAVE_GETUID 1
#define HAVE_GETPASS 1
#define HAVE_SPAWNL 1
#define HAVE_ALARM 1
#define HAVE_ERASECHAR 1
#define HAVE_KILLCHAR 1
#elif defined(_WIN32)
#define HAVE_CURSES_H
#define HAVE_TERM_H
#define HAVE__SPAWNL
#define HAVE_SYS_TYPES_H
#define HAVE_PROCESS_H
#define HAVE_ERASECHAR 1
#define HAVE_KILLCHAR 1
#elif defined(__CYGWIN__)
#define HAVE_SYS_TYPES_H 1
#define HAVE_PWD_H 1
#define HAVE_PWD_H 1
#define HAVE_SYS_UTSNAME_H 1
#define HAVE_ARPA_INET_H 1
#define HAVE_UNISTD_H 1
#define HAVE_TERMIOS_H 1
#define HAVE_NCURSES_TERM_H 1
#define HAVE_ESCDELAY
#define HAVE_SETGID 1
#define HAVE_GETGID 1
#define HAVE_SETUID 1
#define HAVE_GETUID 1
#define HAVE_GETPASS 1
#define HAVE_GETPWUID 1
#define HAVE_WORKING_FORK 1
#define HAVE_ALARM 1
#define HAVE_SPAWNL 1
#define HAVE__SPAWNL 1
#define HAVE_ERASECHAR 1
#define HAVE_KILLCHAR 1
#else /* POSIX */
#define HAVE_SYS_TYPES_H 1
#define HAVE_PWD_H 1
#define HAVE_PWD_H 1
#define HAVE_SYS_UTSNAME_H 1
#define HAVE_ARPA_INET_H 1
#define HAVE_UNISTD_H 1
#define HAVE_TERMIOS_H 1
#define HAVE_TERM_H 1
#define HAVE_SETGID 1
#define HAVE_GETGID 1
#define HAVE_SETUID 1
#define HAVE_GETUID 1
#define HAVE_SETREUID 1
#define HAVE_SETREGID 1
#define HAVE_GETPASS 1
#define HAVE_GETPWUID 1
#define HAVE_WORKING_FORK 1
#define HAVE_ERASECHAR 1
#define HAVE_KILLCHAR 1
#ifndef _AIX
#define HAVE_GETLOADAVG 1
#endif
#define HAVE_ALARM 1
#endif

#ifdef __DJGPP__
#undef HAVE_GETPWUID /* DJGPP's limited version doesn't even work as documented */
#endif

/*
 * Don't change the constants, since they are used for sizes in many
 * places in the program.
 */

#include <stdlib.h>

#undef SIGTSTP

#define MAXSTR		1024	/* maximum length of strings */
#define MAXLINES	32	/* maximum number of screen lines used */
#define MAXCOLS		80	/* maximum number of screen columns used */

#define RN		(((seed = seed*11109+13849) >> 16) & 0xffff)
#ifdef CTRL
#undef CTRL
#endif
#define CTRL(c)		(c & 037)

/*
 * Now all the global variables
 */

extern bool	got_ltc, in_shell;
extern int	wizard;
extern char	fruit[], prbuf[], whoami[];
extern int orig_dsusp;
extern FILE	*scoreboard;

/*
 * Function types
 */

void    auto_save(int);
void	come_down();
void	doctor();
void	end_line();
void    endit(int sig);
void	fatal();
void	getltchars();
void	land();
void    leave(int);
void	my_exit();
void	nohaste();
void	playit();
void    playltchars(void);
void	print_disc(char);
void    quit(int);
void    resetltchars(void);
void	rollwand();
void	runners();
void	set_order();
void	sight();
void	stomach();
void	swander();
void	tstp(int ignored);
void	unconfuse();
void	unsee();
void	visuals();

char	add_line(char *fmt, char *arg);

char	*killname(char monst, bool doart);
char	*nothing(char type);
char	*type_name(int type);

#ifdef CHECKTIME
int	checkout();
#endif

int	md_chmod(char *filename, int mode);
char	*md_crypt(char *key, char *salt);
int	md_dsuspchar();
int	md_erasechar();
char	*md_gethomedir();
char	*md_getusername();
int	md_getuid();
char	*md_getpass(char *prompt);
int	md_getpid();
char	*md_getrealname(int uid);
void	md_init();
int	md_killchar();
void	md_normaluser();
void	md_raw_standout();
void	md_raw_standend();
int	md_readchar();
int	md_setdsuspchar(int c);
int	md_shellescape();
void	md_sleep(int s);
int	md_suspchar();
int	md_hasclreol();
int	md_unlink(char *file);
int	md_unlink_open_file(char *file, FILE *inf);
void md_tstpsignal();
void md_tstphold();
void md_tstpresume();
void md_ignoreallsignals();
void md_onsignal_autosave();
void md_onsignal_exit();
void md_onsignal_default();
int md_issymlink(char *sp);
```

extern.c

```c
/*
 * global variable initializaton
 *
 * @(#)extern.c	4.82 (Berkeley) 02/05/99
 *
 * Rogue: Exploring the Dungeons of Doom
 * Copyright (C) 1980-1983, 1985, 1999 Michael Toy, Ken Arnold and Glenn Wichman
 * All rights reserved.
 *
 * See the file LICENSE.TXT for full copyright and licensing information.
 */

#include <curses.h>
#include "rogue.h"

bool after;				/* True if we want after daemons */
bool again;				/* Repeating the last command */
int  noscore;				/* Was a wizard sometime */
bool seenstairs;			/* Have seen the stairs (for lsd) */
bool amulet = FALSE;			/* He found the amulet */
bool door_stop = FALSE;			/* Stop running when we pass a door */
bool fight_flush = FALSE;		/* True if toilet input */
bool firstmove = FALSE;			/* First move after setting door_stop */
bool got_ltc = FALSE;			/* We have gotten the local tty chars */
bool has_hit = FALSE;			/* Has a "hit" message pending in msg */
bool in_shell = FALSE;			/* True if executing a shell */
bool inv_describe = TRUE;		/* Say which way items are being used */
bool jump = FALSE;			/* Show running as series of jumps */
bool kamikaze = FALSE;			/* to_death really to DEATH */
bool lower_msg = FALSE;			/* Messages should start w/lower case */
bool move_on = FALSE;			/* Next move shouldn't pick up items */
bool msg_esc = FALSE;			/* Check for ESC from msg's --More-- */
bool passgo = FALSE;			/* Follow passages */
bool playing = TRUE;			/* True until he quits */
bool q_comm = FALSE;			/* Are we executing a 'Q' command? */
bool running = FALSE;			/* True if player is running */
bool save_msg = TRUE;			/* Remember last msg */
bool see_floor = TRUE;			/* Show the lamp illuminated floor */
bool stat_msg = FALSE;			/* Should status() print as a msg() */
bool terse = FALSE;			/* True if we should be short */
bool to_death = FALSE;			/* Fighting is to the death! */
bool tombstone = TRUE;			/* Print out tombstone at end */
#ifdef MASTER
int wizard = FALSE;			/* True if allows wizard commands */
#endif
bool pack_used[26] = {			/* Is the character used in the pack? */
    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE,
    FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE
};

char dir_ch;				/* Direction from last get_dir() call */
char file_name[MAXSTR];			/* Save file name */
char huh[MAXSTR];			/* The last message printed */
char *p_colors[MAXPOTIONS];		/* Colors of the potions */
char prbuf[2*MAXSTR];			/* buffer for sprintfs */
char *r_stones[MAXRINGS];		/* Stone settings of the rings */
char runch;				/* Direction player is running */
char *s_names[MAXSCROLLS];		/* Names of the scrolls */
char take;				/* Thing she is taking */
char whoami[MAXSTR];			/* Name of player */
char *ws_made[MAXSTICKS];		/* What sticks are made of */
char *ws_type[MAXSTICKS];		/* Is it a wand or a staff */
int  orig_dsusp;			/* Original dsusp char */
char fruit[MAXSTR] =			/* Favorite fruit */
		{ 's', 'l', 'i', 'm', 'e', '-', 'm', 'o', 'l', 'd', '\0' };
char home[MAXSTR] = { '\0' };		/* User's home directory */
char *inv_t_name[] = {
	"Overwrite",
	"Slow",
	"Clear"
};
char l_last_comm = '\0';		/* Last last_comm */
char l_last_dir = '\0';			/* Last last_dir */
char last_comm = '\0';			/* Last command typed */
char last_dir = '\0';			/* Last direction given */
char *tr_name[] = {			/* Names of the traps */
	"a trapdoor",
	"an arrow trap",
	"a sleeping gas trap",
	"a beartrap",
	"a teleport trap",
	"a poison dart trap",
	"a rust trap",
        "a mysterious trap"
};


int n_objs;				/* # items listed in inventory() call */
int ntraps;				/* Number of traps on this level */
int hungry_state = 0;			/* How hungry is he */
int inpack = 0;				/* Number of things in pack */
int inv_type = 0;			/* Type of inventory to use */
int level = 1;				/* What level she is on */
int max_hit;				/* Max damage done to her in to_death */
int max_level;				/* Deepest player has gone */
int mpos = 0;				/* Where cursor is on top line */
int no_food = 0;			/* Number of levels without food */
int a_class[MAXARMORS] = {		/* Armor class for each armor type */
	8,	/* LEATHER */
	7,	/* RING_MAIL */
	7,	/* STUDDED_LEATHER */
	6,	/* SCALE_MAIL */
	5,	/* CHAIN_MAIL */
	4,	/* SPLINT_MAIL */
	4,	/* BANDED_MAIL */
	3,	/* PLATE_MAIL */
};

int count = 0;				/* Number of times to repeat command */
FILE *scoreboard = NULL;	/* File descriptor for score file */
int food_left;				/* Amount of food in hero's stomach */
int lastscore = -1;			/* Score before this turn */
int no_command = 0;			/* Number of turns asleep */
int no_move = 0;			/* Number of turns held in place */
int purse = 0;				/* How much gold he has */
int quiet = 0;				/* Number of quiet turns */
int vf_hit = 0;				/* Number of time flytrap has hit */

int dnum;				/* Dungeon number */
int seed;				/* Random number seed */
int e_levels[] = {
        10L,
	20L,
	40L,
	80L,
       160L,
       320L,
       640L,
      1300L,
      2600L,
      5200L,
     13000L,
     26000L,
     50000L,
    100000L,
    200000L,
    400000L,
    800000L,
   2000000L,
   4000000L,
   8000000L,
	 0L
};

coord delta;				/* Change indicated to get_dir() */
coord oldpos;				/* Position before last look() call */
coord stairs;				/* Location of staircase */

//✅ PLACE places[MAXLINES*MAXCOLS];		/* level map */

THING *cur_armor;			/* What he is wearing */
THING *cur_ring[2];			/* Which rings are being worn */
THING *cur_weapon;			/* Which weapon he is weilding */
THING *l_last_pick = NULL;		/* Last last_pick */
THING *last_pick = NULL;		/* Last object picked in get_item() */
THING *lvl_obj = NULL;			/* List of objects on this level */
THING *mlist = NULL;			/* List of monsters on the level */
THING player;				/* His stats */
					/* restart of game */

WINDOW *hw = NULL;			/* used as a scratch window */

#define INIT_STATS { 16, 0, 1, 10, 12, "1x4", 12 }

struct stats max_stats = INIT_STATS;	/* The maximum for the player */

struct room *oldrp;			/* Roomin(&oldpos) */
struct room rooms[MAXROOMS];		/* One for each room -- A level */
struct room passages[MAXPASS] =		/* One for each passage */
{
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} },
    { {0, 0}, {0, 0}, {0, 0}, 0, ISGONE|ISDARK, 0, {{0,0}} }
};

#define ___ 1
#define XX 10
struct monster monsters[26] =
    {
/* Name		 CARRY	FLAG    str, exp, lvl, amr, hpt, dmg */
{ "aquator",	   0,	ISMEAN,	{ XX, 20,   5,   2, ___, "0x0/0x0" } },
{ "bat",	   0,	ISFLY,	{ XX,  1,   1,   3, ___, "1x2" } },
{ "centaur",	  15,	0,	{ XX, 17,   4,   4, ___, "1x2/1x5/1x5" } },
{ "dragon",	 100,	ISMEAN,	{ XX,5000, 10,  -1, ___, "1x8/1x8/3x10" } },
{ "emu",	   0,	ISMEAN,	{ XX,  2,   1,   7, ___, "1x2" } },
{ "venus flytrap", 0,	ISMEAN,	{ XX, 80,   8,   3, ___, "%%%x0" } },
	/* NOTE: the damage is %%% so that xstr won't merge this */
	/* string with others, since it is written on in the program */
{ "griffin",	  20,	ISMEAN|ISFLY|ISREGEN, { XX,2000, 13,   2, ___, "4x3/3x5" } },
{ "hobgoblin",	   0,	ISMEAN,	{ XX,  3,   1,   5, ___, "1x8" } },
{ "ice monster",   0,	0,	{ XX,  5,   1,   9, ___, "0x0" } },
{ "jabberwock",   70,	0,	{ XX,3000, 15,   6, ___, "2x12/2x4" } },
{ "kestrel",	   0,	ISMEAN|ISFLY,	{ XX,  1,   1,   7, ___, "1x4" } },
{ "leprechaun",	   0,	0,	{ XX, 10,   3,   8, ___, "1x1" } },
{ "medusa",	  40,	ISMEAN,	{ XX,200,   8,   2, ___, "3x4/3x4/2x5" } },
{ "nymph",	 100,	0,	{ XX, 37,   3,   9, ___, "0x0" } },
{ "orc",	  15,	ISGREED,{ XX,  5,   1,   6, ___, "1x8" } },
{ "phantom",	   0,	ISINVIS,{ XX,120,   8,   3, ___, "4x4" } },
{ "quagga",	   0,	ISMEAN,	{ XX, 15,   3,   3, ___, "1x5/1x5" } },
{ "rattlesnake",   0,	ISMEAN,	{ XX,  9,   2,   3, ___, "1x6" } },
{ "snake",	   0,	ISMEAN,	{ XX,  2,   1,   5, ___, "1x3" } },
{ "troll",	  50,	ISREGEN|ISMEAN,{ XX, 120, 6, 4, ___, "1x8/1x8/2x6" } },
{ "black unicorn", 0,	ISMEAN,	{ XX,190,   7,  -2, ___, "1x9/1x9/2x9" } },
{ "vampire",	  20,	ISREGEN|ISMEAN,{ XX,350,   8,   1, ___, "1x10" } },
{ "wraith",	   0,	0,	{ XX, 55,   5,   4, ___, "1x6" } },
{ "xeroc",	  30,	0,	{ XX,100,   7,   7, ___, "4x4" } },
{ "yeti",	  30,	0,	{ XX, 50,   4,   6, ___, "1x6/1x6" } },
{ "zombie",	   0,	ISMEAN,	{ XX,  6,   2,   8, ___, "1x8" } }
    };
#undef ___
#undef XX

struct obj_info things[NUMTHINGS] = {
    { 0,			26 },	/* potion */
    { 0,			36 },	/* scroll */
    { 0,			16 },	/* food */
    { 0,			 7 },	/* weapon */
    { 0,			 7 },	/* armor */
    { 0,			 4 },	/* ring */
    { 0,			 4 },	/* stick */
};

struct obj_info arm_info[MAXARMORS] = {
    { "leather armor",		 20,	 20, NULL, FALSE },
    { "ring mail",		 15,	 25, NULL, FALSE },
    { "studded leather armor",	 15,	 20, NULL, FALSE },
    { "scale mail",		 13,	 30, NULL, FALSE },
    { "chain mail",		 12,	 75, NULL, FALSE },
    { "splint mail",		 10,	 80, NULL, FALSE },
    { "banded mail",		 10,	 90, NULL, FALSE },
    { "plate mail",		  5,	150, NULL, FALSE },
};
struct obj_info pot_info[MAXPOTIONS] = {
    { "confusion",		 7,   5, NULL, FALSE },
    { "hallucination",		 8,   5, NULL, FALSE },
    { "poison",			 8,   5, NULL, FALSE },
    { "gain strength",		13, 150, NULL, FALSE },
    { "see invisible",		 3, 100, NULL, FALSE },
    { "healing",		13, 130, NULL, FALSE },
    { "monster detection",	 6, 130, NULL, FALSE },
    { "magic detection",	 6, 105, NULL, FALSE },
    { "raise level",		 2, 250, NULL, FALSE },
    { "extra healing",		 5, 200, NULL, FALSE },
    { "haste self",		 5, 190, NULL, FALSE },
    { "restore strength",	13, 130, NULL, FALSE },
    { "blindness",		 5,   5, NULL, FALSE },
    { "levitation",		 6,  75, NULL, FALSE },
};
struct obj_info ring_info[MAXRINGS] = {
    { "protection",		 9, 400, NULL, FALSE },
    { "add strength",		 9, 400, NULL, FALSE },
    { "sustain strength",	 5, 280, NULL, FALSE },
    { "searching",		10, 420, NULL, FALSE },
    { "see invisible",		10, 310, NULL, FALSE },
    { "adornment",		 1,  10, NULL, FALSE },
    { "aggravate monster",	10,  10, NULL, FALSE },
    { "dexterity",		 8, 440, NULL, FALSE },
    { "increase damage",	 8, 400, NULL, FALSE },
    { "regeneration",		 4, 460, NULL, FALSE },
    { "slow digestion",		 9, 240, NULL, FALSE },
    { "teleportation",		 5,  30, NULL, FALSE },
    { "stealth",		 7, 470, NULL, FALSE },
    { "maintain armor",		 5, 380, NULL, FALSE },
};
struct obj_info scr_info[MAXSCROLLS] = {
    { "monster confusion",		 7, 140, NULL, FALSE },
    { "magic mapping",			 4, 150, NULL, FALSE },
    { "hold monster",			 2, 180, NULL, FALSE },
    { "sleep",				 3,   5, NULL, FALSE },
    { "enchant armor",			 7, 160, NULL, FALSE },
    { "identify potion",		10,  80, NULL, FALSE },
    { "identify scroll",		10,  80, NULL, FALSE },
    { "identify weapon",		 6,  80, NULL, FALSE },
    { "identify armor",		 	 7, 100, NULL, FALSE },
    { "identify ring, wand or staff",	10, 115, NULL, FALSE },
    { "scare monster",			 3, 200, NULL, FALSE },
    { "food detection",			 2,  60, NULL, FALSE },
    { "teleportation",			 5, 165, NULL, FALSE },
    { "enchant weapon",			 8, 150, NULL, FALSE },
    { "create monster",			 4,  75, NULL, FALSE },
    { "remove curse",			 7, 105, NULL, FALSE },
    { "aggravate monsters",		 3,  20, NULL, FALSE },
    { "protect armor",			 2, 250, NULL, FALSE },
};
struct obj_info weap_info[MAXWEAPONS + 1] = {
    { "mace",				11,   8, NULL, FALSE },
    { "long sword",			11,  15, NULL, FALSE },
    { "short bow",			12,  15, NULL, FALSE },
    { "arrow",				12,   1, NULL, FALSE },
    { "dagger",				 8,   3, NULL, FALSE },
    { "two handed sword",		10,  75, NULL, FALSE },
    { "dart",				12,   2, NULL, FALSE },
    { "shuriken",			12,   5, NULL, FALSE },
    { "spear",				12,   5, NULL, FALSE },
    { NULL, 0 },	/* DO NOT REMOVE: fake entry for dragon's breath */
};
struct obj_info ws_info[MAXSTICKS] = {
    { "light",			12, 250, NULL, FALSE },
    { "invisibility",		 6,   5, NULL, FALSE },
    { "lightning",		 3, 330, NULL, FALSE },
    { "fire",			 3, 330, NULL, FALSE },
    { "cold",			 3, 330, NULL, FALSE },
    { "polymorph",		15, 310, NULL, FALSE },
    { "magic missile",		10, 170, NULL, FALSE },
    { "haste monster",		10,   5, NULL, FALSE },
    { "slow monster",		11, 350, NULL, FALSE },
    { "drain life",		 9, 300, NULL, FALSE },
    { "nothing",		 1,   5, NULL, FALSE },
    { "teleport away",		 6, 340, NULL, FALSE },
    { "teleport to",		 6,  50, NULL, FALSE },
    { "cancellation",		 5, 280, NULL, FALSE },
};

struct h_list helpstr[] = {
    {'?',	"	prints help",				TRUE},
    {'/',	"	identify object",			TRUE},
    {'h',	"	left",					TRUE},
    {'j',	"	down",					TRUE},
    {'k',	"	up",					TRUE},
    {'l',	"	right",					TRUE},
    {'y',	"	up & left",				TRUE},
    {'u',	"	up & right",				TRUE},
    {'b',	"	down & left",				TRUE},
    {'n',	"	down & right",				TRUE},
    {'H',	"	run left",				FALSE},
    {'J',	"	run down",				FALSE},
    {'K',	"	run up",				FALSE},
    {'L',	"	run right",				FALSE},
    {'Y',	"	run up & left",				FALSE},
    {'U',	"	run up & right",			FALSE},
    {'B',	"	run down & left",			FALSE},
    {'N',	"	run down & right",			FALSE},
    {CTRL('H'),	"	run left until adjacent",		FALSE},
    {CTRL('J'),	"	run down until adjacent",		FALSE},
    {CTRL('K'),	"	run up until adjacent",			FALSE},
    {CTRL('L'),	"	run right until adjacent",		FALSE},
    {CTRL('Y'),	"	run up & left until adjacent",		FALSE},
    {CTRL('U'),	"	run up & right until adjacent",		FALSE},
    {CTRL('B'),	"	run down & left until adjacent",	FALSE},
    {CTRL('N'),	"	run down & right until adjacent",	FALSE},
    {'\0',	"	<SHIFT><dir>: run that way",		TRUE},
    {'\0',	"	<CTRL><dir>: run till adjacent",	TRUE},
    {'f',	"<dir>	fight till death or near death",	TRUE},
    {'t',	"<dir>	throw something",			TRUE},
    {'m',	"<dir>	move onto without picking up",		TRUE},
    {'z',	"<dir>	zap a wand in a direction",		TRUE},
    {'^',	"<dir>	identify trap type",			TRUE},
    {'s',	"	search for trap/secret door",		TRUE},
    {'>',	"	go down a staircase",			TRUE},
    {'<',	"	go up a staircase",			TRUE},
    {'.',	"	rest for a turn",			TRUE},
    {',',	"	pick something up",			TRUE},
    {'i',	"	inventory",				TRUE},
    {'I',	"	inventory single item",			TRUE},
    {'q',	"	quaff potion",				TRUE},
    {'r',	"	read scroll",				TRUE},
    {'e',	"	eat food",				TRUE},
    {'w',	"	wield a weapon",			TRUE},
    {'W',	"	wear armor",				TRUE},
    {'T',	"	take armor off",			TRUE},
    {'P',	"	put on ring",				TRUE},
    {'R',	"	remove ring",				TRUE},
    {'d',	"	drop object",				TRUE},
    {'c',	"	call object",				TRUE},
    {'a',	"	repeat last command",			TRUE},
    {')',	"	print current weapon",			TRUE},
    {']',	"	print current armor",			TRUE},
    {'=',	"	print current rings",			TRUE},
    {'@',	"	print current stats",			TRUE},
    {'D',	"	recall what's been discovered",		TRUE},
    {'o',	"	examine/set options",			TRUE},
    {CTRL('R'),	"	redraw screen",				TRUE},
    {CTRL('P'),	"	repeat last message",			TRUE},
    {ESCAPE,	"	cancel command",			TRUE},
    {'S',	"	save game",				TRUE},
    {'Q',	"	quit",					TRUE},
    {'!',	"	shell escape",				TRUE},
    {'F',	"<dir>	fight till either of you dies",		TRUE},
    {'v',	"	print version number",			TRUE},
    {0,		NULL }
};
```

- [ ] Write document and guide.