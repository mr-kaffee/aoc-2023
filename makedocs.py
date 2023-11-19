"""
Generates static HTML site from solutions and README.adoc files
"""
__version__ = "0.1"
__author__ = "Peter Wieland"


import os
import subprocess
import sys
from dataclasses import dataclass
from urllib.parse import quote


README_FILE = "README.adoc"
DAY_DIR_PREFIX = "day"
GEN_ADOC_DIR = os.path.join("gen", "adoc")


@dataclass
class Solution:
    user: str
    lang: str
    day: int
    dir: str
    readme_file: str = None


def list_solutions(project_dir=os.path.abspath(sys.path[0])):
    """
    Find all solutions in current project directory.

    Parameters:
    project_dir (str, optional): the project directory. Defaults to the directory the interpreter was invoked from.

    Returns:
    list[Solution]: a list of solutions
    """
    solutions = []

    for day_dir_name in os.listdir(project_dir):
        day_dir = os.path.join(project_dir, day_dir_name)
        if not os.path.isdir(day_dir) \
            or not day_dir_name.startswith(DAY_DIR_PREFIX):
            # only process directories whose name starts with "day"
            continue

        # get the day part as int or skip entry
        try:
            day = int(day_dir_name[len(DAY_DIR_PREFIX):])
        except ValueError:
            continue

        for lang in os.listdir(day_dir):
            lang_dir = os.path.join(day_dir, lang)
            if not os.path.isdir(lang_dir):
                # only process directories
                continue

            for user in os.listdir(lang_dir):
                user_dir = os.path.join(lang_dir, user)
                if not os.path.isdir(user_dir):
                    # only process directories
                    continue

                readme_file = README_FILE \
                    if os.path.exists(os.path.join(user_dir, README_FILE)) \
                    else None

                solutions.append(
                    Solution(user, lang, day, user_dir, readme_file))

    return solutions


def write_adoc_files(sols, out_dir: str=GEN_ADOC_DIR):
    """
    Write an ADOC file per user and one summary ADOC file.

    Parameters:
    sols (list[Solution]): list of solutions
    out_dir (str, optional): the folder to generate the ADOC files in. Defaults to GEN_ADOC_DIR
    """

    # create lists per user
    sols_for_users = {}
    for sol in sols:
        sols_for_users.setdefault(sol.user, []).append(sol)

    # determine number of solutions and documented solutions by user
    user_scores = []

    for user, user_sols in sols_for_users.items():
        n_tot = len(user_sols)
        n_doc = len([sol for sol in user_sols if sol.readme_file])
        user_scores.append((user, n_tot, n_doc))

    # sort to have highest number of documented solutions first,
    #    break ties by highest number of total solutions,
    #    break ties by sorting user names alphabetically
    user_scores.sort(key=lambda x: (-x[2], -x[1], x[0]))

    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, "index.adoc"), 'w', encoding="utf-8") as f_sum:
        f_sum.write("= AOC 2023 Solutions\n\n")
        f_sum.write("== Solutions by user\n\n")
        f_sum.write("|===\n")

        for user, n_tot, n_doc in user_scores:
            user_sols = sols_for_users[user]
            f_sum.write(f"| link:user-{quote(user)}.html[{user}] | {n_tot} Solution{'s' if n_tot != 1 else ''} ({n_doc} documented)\n")

            user_file = os.path.join(out_dir, f"user-{user}.adoc")
            with open(user_file, 'w', encoding="utf-8") as f_usr:
                f_usr.write("[[top]]\n")
                f_usr.write(f"= Solutions by {user}\n\n")
                f_usr.write("link:index.html[Overview]\n\n")
                for sol in sorted(user_sols, key=lambda sol: (sol.day, sol.lang)):
                    f_usr.write(f"\n[[sol-{sol.day}]]\n")

                    readme_file = sol.readme_file \
                        if not sol.readme_file or os.path.isabs(sol.readme_file) \
                        else os.path.join(sol.dir, sol.readme_file)
                    if readme_file:
                        f_usr.write(f"include::{readme_file}[leveloffset=0]\n")
                    else:
                        f_usr.write(f"== Undocumented {sol.lang} solution for day {sol.day}\n")
                    f_usr.write("link:#top[Top]\n")

        f_sum.write("|===\n")

        cur_lang = None
        cur_day = None
        f_sum.write("\n== Solutions by language\n\n")
        for sol in sorted(sols, key=lambda sol: (sol.lang, sol.day, sol.user)):
            if cur_lang != sol.lang:
                if cur_lang != None:
                    f_sum.write("|===\n\n")
                f_sum.write(f"=== {sol.lang}\n\n")
                f_sum.write("|===\n")

            f_sum.write(f"| {sol.day if sol.day != cur_day else '':2} | link:user-{quote(sol.user)}.html#sol-{sol.day}[{sol.user}]\n")

            cur_lang = sol.lang
            cur_day = sol.day

        if cur_lang != None:
            f_sum.write("|===\n")

        cur_day = None
        cur_lang = None
        f_sum.write("\n== Solutions by day\n\n")
        for sol in sorted(sols, key=lambda sol: (sol.day, sol.lang, sol.user)):
            if cur_day != sol.day:
                if cur_day != None:
                    f_sum.write("|===\n\n")
                f_sum.write(f"=== Day {sol.day}\n\n")
                f_sum.write("|===\n")

            f_sum.write(f"| {sol.lang if sol.lang != cur_lang else '':10} | link:user-{quote(sol.user)}.html#sol-{sol.day}[{sol.user}]\n")

            cur_day = sol.day
            cur_lang = sol.lang

        if cur_day != None:
            f_sum.write("|===\n")


sols = list_solutions()
write_adoc_files(sols)
subprocess.run(["asciidoctor", "-a", "toc=right", "-a", "Source-highlighter=rouge", "-D", "gen/site", "gen/adoc/*.adoc"])
