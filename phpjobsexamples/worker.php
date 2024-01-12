$worker = new GearmanWorker();
$worker->addServer();

$worker->addFunction('factorial', function (GearmanJob $job) {
    $number = $job->workload();
    $factorial = 1;

    for ($i = 1; $i <= $number; $i++) {
        $factorial *= $i;
    }

    return $factorial;
});

while ($worker->work()) {
    // This loop will keep the worker running, waiting for tasks
}