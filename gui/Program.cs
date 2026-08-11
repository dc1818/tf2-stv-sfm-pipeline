using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Globalization;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace Tf2StvSfmGui
{
    internal static class Program
    {
        [STAThread]
        private static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new MainForm());
        }
    }

    internal sealed class ClipRange
    {
        public long Start;
        public long End;
        public override string ToString()
        {
            return String.Format(CultureInfo.InvariantCulture, "Start {0:N0}   →   End {1:N0}   (10.00 seconds)", Start, End);
        }
    }

    internal sealed class MainForm : Form
    {
        private const long ClipTicks = 667;
        private readonly string root;
        private readonly TabControl tabs = new TabControl();
        private readonly TabPage setupPage = new TabPage("Setup");
        private readonly TabPage pipelinePage = new TabPage("Demo clips");
        private readonly TextBox hlaeBox = new TextBox();
        private readonly TextBox tf2Box = new TextBox();
        private readonly TextBox sfmBox = new TextBox();
        private readonly Label setupState = new Label();
        private readonly Button returnToClipsButton = new Button();
        private readonly ProgressBar setupProgress = new ProgressBar();
        private TextBox setupLog;
        private readonly TextBox demoBox = new TextBox();
        private readonly NumericUpDown newStartTick = new NumericUpDown();
        private readonly Label automaticEnd = new Label();
        private readonly ListBox clipList = new ListBox();
        private readonly TextBox outputBox = new TextBox();
        private TextBox jobLog;
        private readonly ProgressBar progress = new ProgressBar();
        private readonly Label jobStatus = new Label();
        private readonly Button buildButton = new Button();
        private readonly Button cancelButton = new Button();
        private readonly Button openButton = new Button();
        private readonly Button changePathsButton = new Button();
        private readonly List<ClipRange> clips = new List<ClipRange>();
        private readonly object logLock = new object();
        private readonly StringBuilder pendingSetupLog = new StringBuilder();
        private readonly StringBuilder pendingJobLog = new StringBuilder();
        private readonly System.Windows.Forms.Timer logFlushTimer = new System.Windows.Forms.Timer();
        private Process activeProcess;
        private bool busy;
        private bool pipelineShown;
        private bool setupAutomationRunning;
        private string lastBatch;

        public MainForm()
        {
            root = AppDomain.CurrentDomain.BaseDirectory.TrimEnd(Path.DirectorySeparatorChar);
            Text = "TF2 STV to SFM";
            StartPosition = FormStartPosition.CenterScreen;
            MinimumSize = new Size(1020, 720);
            Size = new Size(1180, 860);
            Font = new Font("Segoe UI", 9F);
            BackColor = Color.FromArgb(30, 32, 36);
            ForeColor = Color.Gainsboro;
            AllowDrop = true;
            DragEnter += OnDemoDragEnter;
            DragDrop += OnDemoDragDrop;

            tabs.Dock = DockStyle.Fill;
            setupPage.BackColor = BackColor;
            setupPage.ForeColor = ForeColor;
            pipelinePage.BackColor = BackColor;
            pipelinePage.ForeColor = ForeColor;
            tabs.TabPages.Add(setupPage);
            Controls.Add(tabs);
            BuildSetupPage();
            BuildPipelinePage();
            LoadRememberedPaths();
            RefreshSetupState();
            logFlushTimer.Interval = 200;
            logFlushTimer.Tick += FlushPendingLogs;
            logFlushTimer.Start();
        }

        private static Button Button(string text, int width)
        {
            Button value = new Button();
            value.Text = text;
            value.Width = width;
            value.Height = 36;
            value.FlatStyle = FlatStyle.Flat;
            value.BackColor = Color.FromArgb(53, 90, 135);
            value.ForeColor = Color.White;
            value.FlatAppearance.BorderColor = Color.FromArgb(90, 125, 165);
            return value;
        }

        private static Label Label(string text)
        {
            Label value = new Label();
            value.Text = text;
            value.AutoSize = true;
            value.Margin = new Padding(3, 8, 3, 4);
            return value;
        }

        private static TextBox LogBox()
        {
            TextBox value = new TextBox();
            value.Dock = DockStyle.Fill;
            value.Multiline = true;
            value.ReadOnly = true;
            value.ScrollBars = ScrollBars.Both;
            value.WordWrap = false;
            value.BackColor = Color.FromArgb(17, 18, 20);
            value.ForeColor = Color.FromArgb(218, 224, 230);
            value.Font = new Font("Consolas", 9F);
            return value;
        }

        private void BuildSetupPage()
        {
            TableLayoutPanel layout = new TableLayoutPanel();
            layout.Dock = DockStyle.Fill;
            layout.Padding = new Padding(20);
            layout.ColumnCount = 3;
            layout.RowCount = 10;
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 150));
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 150));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 48));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 48));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 48));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 126));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 52));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 50));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 38));
            layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 24));
            setupPage.Controls.Add(layout);

            layout.Controls.Add(Label("HLAE.exe"), 0, 0);
            hlaeBox.Dock = DockStyle.Fill;
            layout.Controls.Add(hlaeBox, 1, 0);
            Button browse = Button("Choose HLAE.exe", 140);
            browse.Click += BrowseHlae;
            layout.Controls.Add(browse, 2, 0);

            layout.Controls.Add(Label("TF2 folder"), 0, 1);
            tf2Box.Dock = DockStyle.Fill;
            layout.Controls.Add(tf2Box, 1, 1);
            Button tf2Browse = Button("Choose TF2 folder", 140);
            tf2Browse.Click += BrowseTf2;
            layout.Controls.Add(tf2Browse, 2, 1);

            layout.Controls.Add(Label("SFM folder"), 0, 2);
            sfmBox.Dock = DockStyle.Fill;
            layout.Controls.Add(sfmBox, 1, 2);
            Button sfmBrowse = Button("Choose SFM folder", 140);
            sfmBrowse.Click += BrowseSfm;
            layout.Controls.Add(sfmBrowse, 2, 2);

            setupState.Dock = DockStyle.Fill;
            setupState.Font = new Font("Segoe UI", 10F);
            setupState.Padding = new Padding(6);
            layout.SetColumnSpan(setupState, 3);
            layout.Controls.Add(setupState, 0, 3);

            FlowLayoutPanel setupActions = new FlowLayoutPanel();
            setupActions.Dock = DockStyle.Fill;
            returnToClipsButton.Text = "Continue";
            returnToClipsButton.Width = 170;
            returnToClipsButton.Height = 38;
            returnToClipsButton.Visible = false;
            returnToClipsButton.Click += ReturnToClips;
            setupActions.Controls.Add(returnToClipsButton);
            layout.SetColumnSpan(setupActions, 3);
            layout.Controls.Add(setupActions, 0, 4);

            Label note = Label("The app extracts current TF2 models, materials, particles, and sounds only when they are missing. It never mounts your live TF2 folder into SFM.");
            note.MaximumSize = new Size(850, 0);
            note.ForeColor = Color.Silver;
            layout.SetColumnSpan(note, 3);
            layout.Controls.Add(note, 0, 5);

            Label logTitle = Label("Setup log");
            layout.SetColumnSpan(logTitle, 3);
            layout.Controls.Add(logTitle, 0, 6);
            setupLog = LogBox();
            layout.SetColumnSpan(setupLog, 3);
            layout.Controls.Add(setupLog, 0, 7);
            setupProgress.Dock = DockStyle.Fill;
            setupProgress.Style = ProgressBarStyle.Continuous;
            setupProgress.ForeColor = Color.FromArgb(44, 175, 85);
            setupProgress.BackColor = Color.White;
            layout.SetColumnSpan(setupProgress, 3);
            layout.Controls.Add(setupProgress, 0, 8);
        }

        private void BuildPipelinePage()
        {
            TableLayoutPanel layout = new TableLayoutPanel();
            layout.Dock = DockStyle.Fill;
            layout.Padding = new Padding(18);
            layout.ColumnCount = 3;
            layout.RowCount = 9;
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 150));
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
            layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 130));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 46));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 54));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 150));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 48));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 58));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 28));
            layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 48));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 28));
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 24));
            pipelinePage.Controls.Add(layout);

            layout.Controls.Add(Label("STV demo"), 0, 0);
            demoBox.Dock = DockStyle.Fill;
            demoBox.TextChanged += delegate { SuggestOutput(); };
            layout.Controls.Add(demoBox, 1, 0);
            Button browse = Button("Browse demo...", 120);
            browse.Click += BrowseDemo;
            layout.Controls.Add(browse, 2, 0);

            layout.Controls.Add(Label("Starting tick"), 0, 1);
            newStartTick.Maximum = 2000000000;
            newStartTick.ThousandsSeparator = true;
            newStartTick.Dock = DockStyle.Fill;
            newStartTick.ValueChanged += delegate { UpdateAutomaticEnd(); };
            layout.Controls.Add(newStartTick, 1, 1);
            Button add = Button("+ Add clip", 120);
            add.Click += AddClip;
            layout.Controls.Add(add, 2, 1);

            clipList.Dock = DockStyle.Fill;
            clipList.Font = new Font("Consolas", 10F);
            clipList.BackColor = Color.FromArgb(17, 18, 20);
            clipList.ForeColor = Color.Gainsboro;
            layout.SetColumnSpan(clipList, 2);
            layout.Controls.Add(clipList, 0, 2);
            FlowLayoutPanel clipButtons = new FlowLayoutPanel();
            clipButtons.Dock = DockStyle.Fill;
            clipButtons.FlowDirection = FlowDirection.TopDown;
            Button remove = Button("Remove selected", 120);
            remove.Click += RemoveSelectedClip;
            Button clear = Button("Clear clips", 120);
            clear.Click += delegate { clips.Clear(); RefreshClipList(); };
            clipButtons.Controls.Add(remove);
            clipButtons.Controls.Add(clear);
            layout.Controls.Add(clipButtons, 2, 2);

            automaticEnd.ForeColor = Color.LightGreen;
            automaticEnd.Dock = DockStyle.Fill;
            automaticEnd.TextAlign = ContentAlignment.MiddleLeft;
            layout.SetColumnSpan(automaticEnd, 3);
            layout.Controls.Add(automaticEnd, 0, 3);

            layout.Controls.Add(Label("Output location"), 0, 4);
            outputBox.Dock = DockStyle.Fill;
            outputBox.TextChanged += delegate { openButton.Enabled = Directory.Exists(outputBox.Text); };
            layout.Controls.Add(outputBox, 1, 4);
            Button chooseOutput = Button("Select location", 120);
            chooseOutput.Click += BrowseOutput;
            layout.Controls.Add(chooseOutput, 2, 4);

            Label explanation = Label("Each queued capture is limited to a fixed 10-second range (667 ticks). Create SFM clip(s) parses the demo once, then records every listed range through retail TF2 + HLAE. Each clip gets its own SFM-ready sfm_import.agr; shared JSON data is kept once in parsed_demo_data.");
            explanation.MaximumSize = new Size(900, 0);
            explanation.ForeColor = Color.Silver;
            layout.SetColumnSpan(explanation, 3);
            layout.Controls.Add(explanation, 0, 5);

            jobLog = LogBox();
            layout.SetColumnSpan(jobLog, 3);
            layout.Controls.Add(jobLog, 0, 6);

            FlowLayoutPanel actions = new FlowLayoutPanel();
            actions.Dock = DockStyle.Fill;
            buildButton.Text = "Create SFM clip(s)";
            buildButton.Width = 180;
            buildButton.Height = 36;
            buildButton.BackColor = Color.FromArgb(44, 130, 82);
            buildButton.ForeColor = Color.White;
            buildButton.FlatStyle = FlatStyle.Flat;
            buildButton.Click += async delegate { await BuildBatch(); };
            cancelButton.Text = "Cancel";
            cancelButton.Width = 90;
            cancelButton.Height = 36;
            cancelButton.BackColor = Color.FromArgb(44, 130, 82);
            cancelButton.ForeColor = Color.White;
            cancelButton.FlatStyle = FlatStyle.Flat;
            cancelButton.Enabled = false;
            cancelButton.Click += CancelWork;
            openButton.Text = "Open output folder";
            openButton.Width = 145;
            openButton.Height = 36;
            openButton.BackColor = Color.FromArgb(44, 130, 82);
            openButton.ForeColor = Color.White;
            openButton.FlatStyle = FlatStyle.Flat;
            openButton.Enabled = false;
            openButton.Click += OpenBatch;
            changePathsButton.Text = "Change setup paths...";
            changePathsButton.Width = 155;
            changePathsButton.Height = 36;
            changePathsButton.Click += OpenSetupPaths;
            actions.Controls.Add(buildButton);
            actions.Controls.Add(cancelButton);
            actions.Controls.Add(openButton);
            actions.Controls.Add(changePathsButton);
            layout.SetColumnSpan(actions, 3);
            layout.Controls.Add(actions, 0, 7);

            jobStatus.Dock = DockStyle.Fill;
            layout.SetColumnSpan(jobStatus, 3);
            layout.Controls.Add(jobStatus, 0, 8);
            progress.Dock = DockStyle.Fill;
            progress.Style = ProgressBarStyle.Continuous;
            progress.ForeColor = Color.FromArgb(44, 175, 85);
            progress.BackColor = Color.White;
            layout.SetColumnSpan(progress, 3);
            layout.Controls.Add(progress, 0, 9);
            UpdateAutomaticEnd();
        }

        private string SfmRoot()
        {
            return sfmBox.Text.Trim();
        }

        private string Tf2Root()
        {
            return tf2Box.Text.Trim();
        }

        private bool HasValidTf2()
        {
            return File.Exists(Path.Combine(Tf2Root(), "tf_win64.exe")) && File.Exists(Path.Combine(Tf2Root(), "tf", "tf2_misc_dir.vpk"));
        }

        private bool HasValidSfm()
        {
            return File.Exists(Path.Combine(SfmRoot(), "game", "usermod", "gameinfo.txt"));
        }

        private bool HasContent(out int buildables, out int materials, out int particles, out int sounds)
        {
            string fix = Path.Combine(SfmRoot(), "game", "tf_fix");
            string models = Path.Combine(fix, "models", "buildables");
            string material = Path.Combine(fix, "materials");
            string particle = Path.Combine(fix, "particles");
            string sound = Path.Combine(fix, "sound");
            buildables = Directory.Exists(models) ? Directory.GetFiles(models, "*.mdl", SearchOption.AllDirectories).Length : 0;
            materials = Directory.Exists(material) ? Directory.GetFiles(material, "*.*", SearchOption.AllDirectories).Length : 0;
            particles = Directory.Exists(particle) ? Directory.GetFiles(particle, "*.*", SearchOption.AllDirectories).Length : 0;
            sounds = Directory.Exists(sound) ? Directory.GetFiles(sound, "*.*", SearchOption.AllDirectories).Length : 0;
            return buildables > 0 && materials > 0 && particles > 0 && sounds > 0;
        }

        private bool IsTfFixEnabled()
        {
            string gameInfo = Path.Combine(SfmRoot(), "game", "usermod", "gameinfo.txt");
            return File.Exists(gameInfo) && File.ReadAllText(gameInfo).IndexOf("tf_fix", StringComparison.OrdinalIgnoreCase) >= 0;
        }

        private void RefreshSetupState()
        {
            int buildables = 0;
            int materials = 0;
            int particles = 0;
            int sounds = 0;
            bool hasHlae = File.Exists(hlaeBox.Text) && hlaeBox.Text.EndsWith("HLAE.exe", StringComparison.OrdinalIgnoreCase);
            bool hasTf2 = HasValidTf2();
            bool hasSfm = HasValidSfm();
            bool hasContent = hasSfm && HasContent(out buildables, out materials, out particles, out sounds);
            bool enabled = hasSfm && IsTfFixEnabled();
            StringBuilder text = new StringBuilder();
            text.AppendLine((hasHlae ? "PASS" : "NEEDS ACTION") + "  HLAE.exe: " + (hasHlae ? hlaeBox.Text : "choose it above"));
            text.AppendLine((hasTf2 ? "PASS" : "NEEDS ACTION") + "  TF2 folder: " + (hasTf2 ? Tf2Root() : "choose the folder containing tf_win64.exe"));
            text.AppendLine((hasSfm ? "PASS" : "NEEDS ACTION") + "  SFM folder: " + (hasSfm ? SfmRoot() : "choose the folder containing game\\usermod\\gameinfo.txt"));
            text.AppendLine((hasContent ? "PASS" : "NEEDS ACTION") + "  Current TF2 content: " + buildables + " buildable models, " + materials + " material files, " + particles + " particle files, " + sounds + " sound files");
            text.AppendLine((enabled ? "PASS" : "NEEDS ACTION") + "  SFM search path: tf_fix " + (enabled ? "enabled" : "not enabled"));
            setupState.Text = text.ToString();
            setupState.ForeColor = hasHlae && hasTf2 && hasSfm && hasContent && enabled ? Color.LightGreen : Color.Gainsboro;

            if (!hasHlae || !hasTf2 || !hasSfm) {
                returnToClipsButton.Visible = false;
            } else if (!hasContent || !enabled) {
                returnToClipsButton.Visible = false;
                if (!busy && !setupAutomationRunning) {
                    BeginInvoke(new MethodInvoker(StartAutomaticSetup));
                }
            } else {
                returnToClipsButton.Visible = true;
                returnToClipsButton.Enabled = true;
            }
        }

        private void ShowPipeline()
        {
            if (!pipelineShown) {
                tabs.TabPages.Clear();
                tabs.TabPages.Add(pipelinePage);
                tabs.SelectedTab = pipelinePage;
                pipelineShown = true;
            }
        }

        private void OpenSetupPaths(object sender, EventArgs e)
        {
            if (busy) return;
            pipelineShown = false;
            tabs.TabPages.Clear();
            tabs.TabPages.Add(setupPage);
            tabs.SelectedTab = setupPage;
            RefreshSetupState();
        }

        private void ReturnToClips(object sender, EventArgs e)
        {
            ShowPipeline();
        }

        private async Task CompleteSetup()
        {
            int models;
            int materials;
            int particles;
            int sounds;
            bool hasContent = HasContent(out models, out materials, out particles, out sounds);
            BeginSetupWork();
            try {
                if (!hasContent) {
                    await RunWorker("powershell.exe", PowerShellFile("Extract_Current_TF2_Content_For_SFM.ps1", "-IncludeSound -Tf2Root " + Quote(Tf2Root()) + " -SfmRoot " + Quote(SfmRoot())), root, setupLog);
                }
                if (!IsTfFixEnabled()) {
                    await RunWorker("powershell.exe", PowerShellFile("Enable_TF_Fix_In_SFM.ps1", "-SfmRoot " + Quote(SfmRoot())), root, setupLog);
                }
                RefreshSetupState();
                EndSetupWork("Setup completed. The demo clip page is now available.", true);
            }
            catch (Exception ex) { EndSetupWork(ex.Message, false); }
        }

        private async void StartAutomaticSetup()
        {
            if (setupAutomationRunning || busy) return;
            setupAutomationRunning = true;
            try { await CompleteSetup(); }
            finally { setupAutomationRunning = false; }
        }

        private void BeginSetupWork()
        {
            busy = true;
            setupLog.Clear();
            lock (logLock) { pendingSetupLog.Length = 0; }
            setupProgress.Value = 0;
            setupState.Text = "Working... the log below will update as files are extracted.";
        }

        private void EndSetupWork(string message, bool success)
        {
            if (InvokeRequired) { Invoke(new Action<string, bool>(EndSetupWork), message, success); return; }
            busy = false;
            setupProgress.Value = success ? 100 : 0;
            Append(setupLog, "\r\n" + (success ? "SUCCESS: " : "ERROR: ") + message + "\r\n");
            RefreshSetupState();
        }

        private async Task BuildBatch()
        {
            if (busy) return;
            if (!File.Exists(demoBox.Text) || !demoBox.Text.EndsWith(".dem", StringComparison.OrdinalIgnoreCase)) { MessageBox.Show(this, "Choose an existing .dem file.", Text); return; }
            if (clips.Count == 0) { MessageBox.Show(this, "Add at least one 10-second clip start tick.", Text); return; }
            if (String.IsNullOrWhiteSpace(outputBox.Text)) { MessageBox.Show(this, "Choose an output location.", Text); return; }
            BeginJob();
            try {
                string batchRoot = Path.Combine(outputBox.Text, Path.GetFileNameWithoutExtension(demoBox.Text) + "_sfm_batch_" + DateTime.Now.ToString("yyyyMMdd_HHmmss"));
                Directory.CreateDirectory(batchRoot);
                Append(jobLog, "Output batch: " + batchRoot + "\r\n");
                string parserDirectory = Path.Combine(batchRoot, "parsed_demo_data");
                Directory.CreateDirectory(parserDirectory);
                await RunParser(parserDirectory);
                SetProgress(20, "Parser complete. Recording " + clips.Count + " clip(s)...");
                for (int i = 0; i < clips.Count; ++i) {
                    ClipRange clip = clips[i];
                    string clipDirectory = Path.Combine(batchRoot, "clips", String.Format(CultureInfo.InvariantCulture, "clip_{0:000}_{1}-{2}", i + 1, clip.Start, clip.End));
                    Directory.CreateDirectory(clipDirectory);
                    SetProgress(20 + (int)(70.0 * i / clips.Count), "Recording clip " + (i + 1) + " of " + clips.Count + "...");
                    await RunCapture(clipDirectory, clip);
                    await RunFinalizer(clipDirectory, parserDirectory);
                    SetProgress(20 + (int)(70.0 * (i + 1) / clips.Count), "Finished clip " + (i + 1) + " of " + clips.Count + ".");
                }
                File.WriteAllText(Path.Combine(batchRoot, "batch.txt"), "Parsed data: parsed_demo_data\r\nClips: clips\r\n");
                lastBatch = batchRoot;
                FinishJob("Completed " + clips.Count + " SFM-ready clip project(s).", true);
            }
            catch (Exception ex) { FinishJob(ex.Message, false); }
        }

        private Task<int> RunParser(string parserDirectory)
        {
            string executable = Path.Combine(root, "parser", "target", "release", "export_all.exe");
            if (!File.Exists(executable)) throw new FileNotFoundException("Parser is missing. Run Build_GUI_And_Parser.bat once.", executable);
            Append(jobLog, "[1/3] Parsing all available STV data once...\r\n");
            return RunWorker(executable, Quote(demoBox.Text) + " " + Quote(parserDirectory), root, jobLog);
        }

        private Task<int> RunCapture(string clipDirectory, ClipRange clip)
        {
            File.WriteAllText(Path.Combine(root, "HLAE_PATH.txt"), hlaeBox.Text.Trim() + Environment.NewLine);
            string script = Path.Combine(root, "tools", "Run_HLAE_AGR_Capture.ps1");
            string extra = "-DemoPath " + Quote(demoBox.Text) + " -ProjectDirectory " + Quote(clipDirectory) +
                " -StartTick " + clip.Start.ToString(CultureInfo.InvariantCulture) + " -EndTick " + clip.End.ToString(CultureInfo.InvariantCulture) + " -HlaePath " + Quote(hlaeBox.Text.Trim());
            extra += " -Tf2Root " + Quote(Tf2Root());
            Append(jobLog, "\r\n[2/3] HLAE capture: " + clip + "\r\n");
            return RunWorker("powershell.exe", PowerShellFile("Run_HLAE_AGR_Capture.ps1", extra), root, jobLog);
        }

        private Task<int> RunFinalizer(string clipDirectory, string parserDirectory)
        {
            string extra = "-ProjectDirectory " + Quote(clipDirectory) + " -SourceDemo " + Quote(demoBox.Text) + " -ParserDirectory " + Quote(parserDirectory);
            Append(jobLog, "[3/3] Writing combined project metadata...\r\n");
            return RunWorker("powershell.exe", PowerShellFile("Finalize_HLAE_Project.ps1", extra), root, jobLog);
        }

        private string PowerShellFile(string fileName, string extra)
        {
            return "-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File " + Quote(Path.Combine(root, "tools", fileName)) + " " + extra;
        }

        private Task<int> RunWorker(string fileName, string arguments, string workDir, TextBox target)
        {
            return Task.Run<int>(() => {
                ProcessStartInfo info = new ProcessStartInfo();
                info.FileName = fileName;
                info.Arguments = arguments;
                info.WorkingDirectory = workDir;
                info.UseShellExecute = false;
                info.CreateNoWindow = true;
                info.RedirectStandardOutput = true;
                info.RedirectStandardError = true;
                Process process = new Process();
                process.StartInfo = info;
                process.OutputDataReceived += delegate(object sender, DataReceivedEventArgs e) { if (e.Data != null) HandleWorkerOutput(target, e.Data); };
                process.ErrorDataReceived += delegate(object sender, DataReceivedEventArgs e) { if (e.Data != null) HandleWorkerOutput(target, e.Data); };
                activeProcess = process;
                process.Start();
                process.BeginOutputReadLine();
                process.BeginErrorReadLine();
                process.WaitForExit();
                int code = process.ExitCode;
                activeProcess = null;
                process.Dispose();
                if (code != 0) throw new InvalidOperationException(Path.GetFileName(fileName) + " exited with code " + code + ". See the log." );
                return code;
            });
        }

        private void BeginJob()
        {
            busy = true;
            jobLog.Clear();
            lock (logLock) { pendingJobLog.Length = 0; }
            buildButton.Enabled = false;
            cancelButton.Enabled = true;
            openButton.Enabled = false;
            SetProgress(0, "Starting batch project...");
        }

        private void FinishJob(string message, bool success)
        {
            if (InvokeRequired) { Invoke(new Action<string, bool>(FinishJob), message, success); return; }
            busy = false;
            buildButton.Enabled = true;
            cancelButton.Enabled = false;
            if (success) {
                openButton.Enabled = Directory.Exists(lastBatch);
                SetProgress(100, message);
                Append(jobLog, "\r\nSUCCESS: " + message + "\r\n");
            } else {
                progress.Value = 0;
                openButton.Enabled = Directory.Exists(outputBox.Text);
                jobStatus.Text = "Failed: " + message;
                jobStatus.ForeColor = Color.OrangeRed;
                Append(jobLog, "\r\nERROR: " + message + "\r\n");
            }
        }

        private void SetProgress(int value, string text)
        {
            if (InvokeRequired) { Invoke(new Action<int, string>(SetProgress), value, text); return; }
            progress.Style = ProgressBarStyle.Continuous;
            progress.Value = Math.Max(0, Math.Min(100, value));
            jobStatus.Text = text;
            jobStatus.ForeColor = Color.Gainsboro;
        }

        private void AddClip(object sender, EventArgs e)
        {
            long start = Decimal.ToInt64(newStartTick.Value);
            foreach (ClipRange item in clips) if (item.Start == start) return;
            clips.Add(new ClipRange { Start = start, End = start + ClipTicks });
            clips.Sort(delegate(ClipRange a, ClipRange b) { return a.Start.CompareTo(b.Start); });
            RefreshClipList();
        }

        private void RemoveSelectedClip(object sender, EventArgs e)
        {
            ClipRange selected = clipList.SelectedItem as ClipRange;
            if (selected != null) { clips.Remove(selected); RefreshClipList(); }
        }

        private void RefreshClipList()
        {
            clipList.DataSource = null;
            clipList.DataSource = clips;
        }

        private void UpdateAutomaticEnd()
        {
            long start = Decimal.ToInt64(newStartTick.Value);
            automaticEnd.Text = "Automatic end tick: " + (start + ClipTicks).ToString("N0", CultureInfo.InvariantCulture) + "  —  fixed 10.00-second clip (667 ticks).";
        }

        private void BrowseDemo(object sender, EventArgs e)
        {
            using (OpenFileDialog dialog = new OpenFileDialog()) {
                dialog.Filter = "TF2 demo (*.dem)|*.dem|All files (*.*)|*.*";
                if (dialog.ShowDialog(this) == DialogResult.OK) demoBox.Text = dialog.FileName;
            }
        }

        private void BrowseOutput(object sender, EventArgs e)
        {
            using (FolderBrowserDialog dialog = new FolderBrowserDialog()) {
                if (Directory.Exists(outputBox.Text)) dialog.SelectedPath = outputBox.Text;
                if (dialog.ShowDialog(this) == DialogResult.OK) {
                    outputBox.Text = dialog.SelectedPath;
                    openButton.Enabled = true;
                }
            }
        }

        private void BrowseHlae(object sender, EventArgs e)
        {
            using (OpenFileDialog dialog = new OpenFileDialog()) {
                dialog.Filter = "HLAE executable (HLAE.exe)|HLAE.exe|Executable (*.exe)|*.exe";
                if (dialog.ShowDialog(this) != DialogResult.OK) return;
                hlaeBox.Text = dialog.FileName;
                File.WriteAllText(Path.Combine(root, "HLAE_PATH.txt"), dialog.FileName + Environment.NewLine);
                RefreshSetupState();
            }
        }

        private void BrowseTf2(object sender, EventArgs e)
        {
            using (FolderBrowserDialog dialog = new FolderBrowserDialog()) {
                dialog.Description = "Choose the Team Fortress 2 folder containing tf_win64.exe";
                if (Directory.Exists(tf2Box.Text)) dialog.SelectedPath = tf2Box.Text;
                if (dialog.ShowDialog(this) != DialogResult.OK) return;
                tf2Box.Text = dialog.SelectedPath;
                File.WriteAllText(Path.Combine(root, "TF2_ROOT.txt"), tf2Box.Text + Environment.NewLine);
                RefreshSetupState();
            }
        }

        private void BrowseSfm(object sender, EventArgs e)
        {
            using (FolderBrowserDialog dialog = new FolderBrowserDialog()) {
                dialog.Description = "Choose the SourceFilmmaker folder containing game\\usermod";
                if (Directory.Exists(sfmBox.Text)) dialog.SelectedPath = sfmBox.Text;
                if (dialog.ShowDialog(this) != DialogResult.OK) return;
                sfmBox.Text = dialog.SelectedPath;
                File.WriteAllText(Path.Combine(root, "SFM_ROOT.txt"), sfmBox.Text + Environment.NewLine);
                RefreshSetupState();
            }
        }

        private void LoadRememberedPaths()
        {
            string hlae = Path.Combine(root, "HLAE_PATH.txt");
            string tf2 = Path.Combine(root, "TF2_ROOT.txt");
            string sfm = Path.Combine(root, "SFM_ROOT.txt");
            if (File.Exists(hlae)) hlaeBox.Text = File.ReadAllText(hlae).Trim();
            if (File.Exists(tf2)) tf2Box.Text = File.ReadAllText(tf2).Trim();
            else tf2Box.Text = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFilesX86), "Steam", "steamapps", "common", "Team Fortress 2");
            if (File.Exists(sfm)) sfmBox.Text = File.ReadAllText(sfm).Trim();
            else sfmBox.Text = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFilesX86), "Steam", "steamapps", "common", "SourceFilmmaker");
        }

        private void SuggestOutput()
        {
            if (!File.Exists(demoBox.Text)) return;
            outputBox.Text = Path.GetDirectoryName(demoBox.Text);
            openButton.Enabled = Directory.Exists(outputBox.Text);
        }

        private void OnDemoDragEnter(object sender, DragEventArgs e)
        {
            if (e.Data.GetDataPresent(DataFormats.FileDrop)) e.Effect = DragDropEffects.Copy;
        }

        private void OnDemoDragDrop(object sender, DragEventArgs e)
        {
            string[] files = e.Data.GetData(DataFormats.FileDrop) as string[];
            if (files != null && files.Length > 0 && files[0].EndsWith(".dem", StringComparison.OrdinalIgnoreCase)) demoBox.Text = files[0];
        }

        private void CancelWork(object sender, EventArgs e)
        {
            Process process = activeProcess;
            if (process == null || process.HasExited) return;
            cancelButton.Enabled = false;
            jobStatus.Text = "Cancelling worker and its HLAE/TF2 child processes...";
            int processId = process.Id;
            Task.Run(delegate { KillProcessTree(processId); });
            Append(jobLog, "\r\nCancellation requested. Stopping the complete worker process tree...\r\n");
        }

        private static void KillProcessTree(int processId)
        {
            try {
                ProcessStartInfo info = new ProcessStartInfo();
                info.FileName = "taskkill.exe";
                info.Arguments = "/PID " + processId.ToString(CultureInfo.InvariantCulture) + " /T /F";
                info.UseShellExecute = false;
                info.CreateNoWindow = true;
                using (Process killer = Process.Start(info)) { killer.WaitForExit(10000); }
            }
            catch { }
        }

        private void OpenBatch(object sender, EventArgs e)
        {
            string folder = !String.IsNullOrEmpty(lastBatch) && Directory.Exists(lastBatch) ? lastBatch : outputBox.Text;
            if (Directory.Exists(folder)) Process.Start("explorer.exe", Quote(folder));
            else MessageBox.Show(this, "Select an existing output location first.", Text, MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        private void Append(TextBox box, string text)
        {
            lock (logLock) {
                if (box == setupLog) pendingSetupLog.Append(text);
                else pendingJobLog.Append(text);
            }
        }

        private void FlushPendingLogs(object sender, EventArgs e)
        {
            string setupText = null;
            string jobText = null;
            lock (logLock) {
                if (pendingSetupLog.Length > 0) { setupText = pendingSetupLog.ToString(); pendingSetupLog.Length = 0; }
                if (pendingJobLog.Length > 0) { jobText = pendingJobLog.ToString(); pendingJobLog.Length = 0; }
            }
            if (setupText != null) FlushTextBox(setupLog, setupText);
            if (jobText != null) FlushTextBox(jobLog, jobText);
        }

        private static void FlushTextBox(TextBox box, string text)
        {
            if (box == null || box.IsDisposed) return;
            const int maximumCharacters = 1500000;
            const int retainedCharacters = 1000000;
            if (box.TextLength + text.Length > maximumCharacters && box.TextLength > retainedCharacters) {
                box.Text = "[Older console output trimmed to keep the GUI responsive.]\r\n" + box.Text.Substring(box.TextLength - retainedCharacters);
            }
            box.AppendText(text);
            box.SelectionStart = box.TextLength;
            box.ScrollToCaret();
        }

        private void HandleWorkerOutput(TextBox target, string line)
        {
            Append(target, line + "\r\n");
            if (target != setupLog) return;
            Match match = Regex.Match(line, @"TF2SFM_PROGRESS:\s*(\d+)");
            if (!match.Success) return;
            int value;
            if (Int32.TryParse(match.Groups[1].Value, out value)) SetSetupProgress(value);
        }

        private void SetSetupProgress(int value)
        {
            if (setupProgress.InvokeRequired) { setupProgress.BeginInvoke(new Action<int>(SetSetupProgress), value); return; }
            setupProgress.Value = Math.Max(0, Math.Min(100, value));
        }

        private static string Quote(string value)
        {
            return "\"" + value.Replace("\"", "\\\"") + "\"";
        }
    }
}
