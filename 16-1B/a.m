
function y = f(x)
  y = zeros(1, 26);
  for i=1:length(x)
    c = x(i) - 'A';
    y(c) = y(c) + 1;
  end
end


matrix = zeros(10, 26);

numbers = {"ZERO" "ONE" "TWO" "THREE" "FOUR" "FIVE" "SIX" "SEVEN" "EIGHT" "NINE"};
for i=1:10
  number = numbers{1, i};
  matrix(i, :) = f(number);
end

n = input("");
for i=1:n
  s = input("", "s");
  m = f(s);

  k = matrix' \ m';

  printf("Case #%d: ", i);
  for x = 1:length(k)
    y = round(k(x));
    for z = 1:y
      printf("%d", x-1);
    end
  end

  printf("\n");
end
