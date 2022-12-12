package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

type File struct {
	Name string
	Size uint64
}

type Directory struct {
	Name        string
	Files       []File
	Directories []Directory
	Parent      *Directory
}

func find_or_create_dir(current_dir *Directory, dir_name string) *Directory {
	idx := slices.IndexFunc(current_dir.Directories, func(d Directory) bool { return d.Name == dir_name })

	if idx >= 0 {
		return &current_dir.Directories[idx]
	} else {
		new_dir := Directory{Name: dir_name, Files: []File{}, Directories: []Directory{}, Parent: current_dir}
		current_dir.Directories = append(current_dir.Directories, new_dir)
		return &new_dir
	}
}

func find_or_create_file(current_dir *Directory, file_name string, size uint64) File {
	idx := slices.IndexFunc(current_dir.Files, func(d File) bool { return d.Name == file_name })

	if idx >= 0 {
		return current_dir.Files[idx]
	} else {
		new_file := File{Name: file_name, Size: size}
		current_dir.Files = append(current_dir.Files, new_file)
		return new_file
	}
}

func map_out_tree(lines []string, root_dir *Directory) {
	cd_re := regexp.MustCompile(`\$ cd ([\w./]+)`)
	dir_re := regexp.MustCompile(`dir ([\w./]+)`)
	file_re := regexp.MustCompile(`(\d+) (.+)`)

	current_dir := root_dir

	for _, line := range lines {
		if cd_re.MatchString(line) {
			dir_name := cd_re.FindStringSubmatch(line)[1]
			if dir_name == "/" {
				current_dir = root_dir
			} else if dir_name == ".." {
				current_dir = current_dir.Parent
			} else {
				current_dir = find_or_create_dir(current_dir, dir_name)
			}
		} else if dir_re.MatchString(line) {
			dir_name := dir_re.FindStringSubmatch(line)[1]
			find_or_create_dir(current_dir, dir_name)
		} else if file_re.MatchString(line) {
			matches := file_re.FindStringSubmatch(line)
			size, _ := strconv.ParseUint(matches[1], 10, 64)
			file_name := matches[2]
			find_or_create_file(current_dir, file_name, size)
		}
	}
}

func calculate_the_sizes(dir Directory, all_directories map[string]uint64, lineage string) string {
	this_lineage := lineage + "/" + dir.Name
	for _, file := range dir.Files {
		all_directories[this_lineage] += file.Size
	}
	for _, d := range dir.Directories {
		new_lineage := calculate_the_sizes(d, all_directories, this_lineage)
		all_directories[this_lineage] += all_directories[new_lineage]
	}
	return this_lineage
}

func find_sum_of_directories(all_directories map[string]uint64) uint64 {
	var LIMIT uint64 = 100000
	var sum uint64
	for _, size := range all_directories {
		if size < LIMIT {
			sum += size
		}
	}
	return sum
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	root_dir := Directory{Name: "/", Files: []File{}, Directories: []Directory{}}
	root_dir.Parent = &root_dir

	map_out_tree(lines, &root_dir)

	all_directories := make(map[string]uint64)
	calculate_the_sizes(root_dir, all_directories, "")

	fmt.Println(find_sum_of_directories(all_directories))

}
