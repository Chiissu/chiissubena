package cmd

import (
	"fmt"
	"os"

	"github.com/manifoldco/promptui"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "Notabena",
	Short: "Productivity, improved",
	Long:  `The clutter-free open-source note-taking app, built with Go`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Welcome to Notabena!")
		options := []string{"New note", "View note", "Edit note", "Delete note", "About", "Exit"}
		prompt := promptui.Select{
			Label: "What do you want to do?",
			Items: options,
		}

		_, result, err := prompt.Run()

		if err != nil {
			fmt.Printf("Prompt failed %v\n", err)
			return
		}

		switch result {
		case options[0]:
			fmt.Println("you have chosen to create new note wow")
		case options[1]:
			fmt.Println("you have chosen to view note")
		case options[2]:
			fmt.Println("you have chosen to edit note")
		case options[3]:
			fmt.Println("you have chosen to delete note")
		case options[4]:
			fmt.Println("0.3 poc")
		case options[5]:
			return
		}
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
