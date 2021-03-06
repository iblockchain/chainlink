package main

import (
	"os"
	"time"

	"github.com/smartcontractkit/chainlink/cmd"
	"github.com/smartcontractkit/chainlink/store"
	"github.com/urfave/cli"
)

func init() {
	time.LoadLocation("UTC")
}

func main() {
	Run(NewProductionClient(), os.Args...)
}

// Run runs the CLI, providing further command instructions by default.
func Run(client *cmd.Client, args ...string) {
	app := cli.NewApp()
	app.Usage = "CLI for Chainlink"
	app.Version = store.Version
	app.Flags = []cli.Flag{
		cli.BoolFlag{
			Name:  "json, j",
			Usage: "json output as opposed to table",
		},
	}
	app.Before = func(c *cli.Context) error {
		if c.Bool("json") {
			client.Renderer = cmd.RendererJSON{Writer: os.Stdout}
		}
		return nil
	}
	app.Commands = []cli.Command{
		{
			Name:    "node",
			Aliases: []string{"n"},
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "password, p",
					Usage: "password for the node's account",
				},
				cli.BoolFlag{
					Name:  "debug, d",
					Usage: "set logger level to debug",
				},
			},
			Usage:  "Run the chainlink node",
			Action: client.RunNode,
		},
		{
			Name:    "jobspecs",
			Aliases: []string{"jobs", "j", "specs"},
			Usage:   "Get all jobs",
			Action:  client.GetJobSpecs,
			Flags: []cli.Flag{
				cli.IntFlag{
					Name:  "page",
					Usage: "page of results to display",
				},
			},
		},
		{
			Name:    "show",
			Aliases: []string{"s"},
			Usage:   "Show a specific job",
			Action:  client.ShowJobSpec,
		},
		{
			Name:    "create",
			Aliases: []string{"c"},
			Usage:   "Create job spec from JSON",
			Action:  client.CreateJobSpec,
		},
		{
			Name:    "run",
			Aliases: []string{"r"},
			Usage:   "Begin job run for specid",
			Action:  client.CreateJobRun,
		},
		{
			Name:   "backup",
			Usage:  "Backup the database of the running node",
			Action: client.BackupDatabase,
		},
		{
			Name:    "import",
			Aliases: []string{"i"},
			Usage:   "Import a key file to use with the node",
			Action:  client.ImportKey,
		},
		{
			Name:   "bridge",
			Usage:  "Add a new bridge to the node",
			Action: client.AddBridge,
		},
		{
			Name:   "getbridges",
			Usage:  "List all bridges added to the node",
			Action: client.GetBridges,
			Flags: []cli.Flag{
				cli.IntFlag{
					Name:  "page",
					Usage: "page of results to display",
				},
			},
		},
		{
			Name:   "showbridge",
			Usage:  "Show a specific bridge",
			Action: client.ShowBridge,
		},
	}
	app.Run(args)
}

// NewProductionClient configures an instance of the CLI to be used
// in production.
func NewProductionClient() *cmd.Client {
	return &cmd.Client{
		Renderer:   cmd.RendererTable{Writer: os.Stdout},
		Config:     store.NewConfig(),
		AppFactory: cmd.ChainlinkAppFactory{},
		Auth:       cmd.TerminalAuthenticator{Prompter: cmd.PasswordPrompter{}, Exiter: os.Exit},
		Runner:     cmd.ChainlinkRunner{},
	}
}
